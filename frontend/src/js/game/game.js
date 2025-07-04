import * as THREE from 'three';
import { Card, CardPile, getCardTexturePathById, OpponentCard } from './cards';
import { ServerConnexion } from '../server/server_connection';
import { Opponent, Player } from './player';
import { PlayerUI } from '../ui/player_ui';
import { CSS2DRenderer, OrbitControls } from 'three-stdlib';
import { degToRad } from 'three/src/math/MathUtils';
import { CardTooltip } from '../ui/card_tooltip';
import { ActionTypeDTO, ChangeTurnResponse, CollectDiscardCardsResponse, DrawCardResponse, GameEndResponse, GameStatusResponse, PlayCardResponse, SessionInfoResponse } from '../server/dto';
import { EventMgr } from './events/event_mgr';
import { ChangeTurnEvent, DamagePlayerEvent, DrawCardEvent, GameUpdateEvent, HealPlayerEvent, PutCardInPile, PutCardForward, ThrowDiceEvent, CollectDiscardCardsEvent, GameEndEvent } from './events/events';
import { displayPopup } from '../ui/popup';
import { CardKind } from './database';

/** @type {THREE.Scene | null} */
export let scene;
/** @type {THREE.PerspectiveCamera | null} */
export let camera;
/** @type {THREE.WebGLRenderer | null} */
export let renderer;
/** @type {CSS2DRenderer | null} */
export let labelRenderer;

export let raycaster = new THREE.Raycaster();
export let pointer = new THREE.Vector2();

/** @type {Player | null} */
export let PLAYER;
/** @type {Map<number, Opponent>} */
export let OPPONENTS = new Map();

export let current_player_turn;

/**
* @typedef {{
*  id: any
*  name: string
* }}
* PlayerProfile
* 
* @typedef {{
*  id: any
*  players: Array<PlayerProfile>
* }}
* SessionInfo
* 
* @type {SessionInfo | null}
*/
export let session_info;

/** @type {CardPile | null} */
export let cardPile;

/** @type {ServerConnexion | null} */
export let serverConnexion;

/** @type {CardTooltip | null} */
export let cardTooltip;

/** @type {EventMgr | null} */
export let eventMgr;

/** @type {number | null} */
export let winner_id = null;


export function initGame() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color().setHex(0xf0f0f0);
    camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize( window.innerWidth, window.innerHeight );
    // renderer.setPixelRatio( 2.0 );
    renderer.setPixelRatio( window.devicePixelRatio );
    document.body.appendChild( renderer.domElement );

    // debug
    // const controls = new OrbitControls( camera, renderer.domElement );

    labelRenderer = new CSS2DRenderer();
    labelRenderer.setSize( window.innerWidth, window.innerHeight );
    labelRenderer.domElement.id = "canvas-ui";
    labelRenderer.domElement.style.position = 'absolute';
    labelRenderer.domElement.style.top = '0px';
    labelRenderer.domElement.style.pointerEvents = "none";
    document.body.appendChild( labelRenderer.domElement );

    // camera.position.set(0, 6, 8);
    // camera.lookAt(new THREE.Vector3(0, 3, 3));
    camera.position.set(0, 10, 2);
    camera.lookAt(new THREE.Vector3(0, 0, 1));

    // lights

    const ambientLight = new THREE.AmbientLight( 0x606060, 3 );
    ambientLight.position.set(-8, 20, -8);
    scene.add( ambientLight );

    const directionalLight = new THREE.DirectionalLight( 0xffffff, 3 );
    directionalLight.position.set( 1, 0.75, 0.5 ).normalize();
    scene.add( directionalLight );

    // objects

    const boardTexture = new THREE.TextureLoader().load('assets/board.jpg');
    const boardMaterial = new THREE.MeshStandardMaterial({ map: boardTexture });
    const boardGeometry = new THREE.PlaneGeometry(10, 10);
    const board = new THREE.Mesh(boardGeometry, boardMaterial);
    board.rotation.x = -Math.PI / 2;
    scene.add(board);

    cardPile = new CardPile();
    scene.add(cardPile);

    PLAYER = new Player(scene);
    PLAYER.position.set(0, 2, 5);
    const playerUI = new PlayerUI(PLAYER);
    playerUI.position.set(-4, 0, 0);

    // Interactions

    renderer.domElement.addEventListener( 'pointermove', onPointerMove );
    renderer.domElement.addEventListener( 'pointerdown', onPointerDown );

    window.addEventListener( 'resize', onWindowResize );

    const gridHelper = new THREE.GridHelper(10, 10);
    scene.add(gridHelper);

    cardTooltip = new CardTooltip(scene);
    cardTooltip.visible = false;

    renderSceneView();

    eventMgr = new EventMgr();

    serverConnexion = new ServerConnexion();

    // connect server events
    serverConnexion.addEventListener("connectionchange", ev => {
        console.log("Connection changed, status:", ev.detail.status);
    })
    serverConnexion.addEventListener("gameupdate", upd => {
        onServerUpdate(upd.detail);
    })
    serverConnexion.addEventListener("chatmessage", ev => {
        console.log("Chat message received:", ev.detail.message);
    })
    serverConnexion.addEventListener("sessioninfo", ev => {
        onSessionInfoReceived(ev.detail);
    })
    serverConnexion.addEventListener("playcard", ev => {
        onPlayCardEvent(ev.detail);
    })
    serverConnexion.addEventListener("drawcard", ev => {
        onDrawCardEvent(ev.detail);
    })
    serverConnexion.addEventListener("changeturn", ev => {
        onChangeTurnEvent(ev.detail);
    })
    serverConnexion.addEventListener("collectdiscardcards", ev => {
        onCollectDiscardCardsEvent(ev.detail);
    })
    serverConnexion.addEventListener("gameend", ev => {
        onGameEndEvent(ev.detail);
    })
}


export function connectToServer(wsUrl) {
    serverConnexion.connect(wsUrl);
}


/** @param {GameStatusResponse} data  */
export function onServerUpdate(data) {
    console.log("Game Update:", data);

    eventMgr.pushEvent(new GameUpdateEvent(data));
}


/**
 * setup expected player count, names, and identifiers
 * @param {SessionInfoResponse} info
 */
export function onSessionInfoReceived(info) {
    const my_id = info.id;
    let my_profile = null;
    const opponents_profile = [];

    const IDs = new Set();

    for (let i = 0; i < info.players.length; i++) {
        const profile = info.players[i];

        if (IDs.has(profile.id))
            throw new Error("Duplicate player id found when parsing session info !");

        IDs.add(profile.id);

        if (profile.id == my_id) {
            my_profile = profile;
        } else {
            opponents_profile.push(profile);
        }
    }

    if (my_profile == null)
        throw new Error("Player profile not in array !");

    if (opponents_profile.length < 1)
        throw new Error("Not enought opponents !");

    PLAYER.name = my_profile.name;

    opponents_profile.forEach(profile => {
        const opponent = new Opponent(scene);
        const ui = new PlayerUI(opponent);
        ui.position.set(-1, 2, 0);
        opponent.name = profile.name;
        OPPONENTS.set(profile.id, opponent);
    });

    session_info = info;

    updateOpponentPositions();

    OPPONENTS.forEach(opponent => {
        opponent.updateHandCardPositions();
    });
}

/** @param {PlayCardResponse} data  */
export function onPlayCardEvent(data) {
    const events = [];

    const player = getPlayerById(data.player_id);
    const card_id = data.card_id;
    const hand_index = data.hand_index;

    let card;

    // get card from player hand or create it if it doesn't exist or doesn't match (desync?)
    if (player.isCardInHand(card_id, hand_index)) {
        card = player.cards[hand_index];
    } else {
        if (player instanceof Opponent)
            card = new OpponentCard();
        else
            card = new Card(card_id);
        scene.add(card);
        const { x, y, z } = player.getHandCardPositionByIndex(hand_index);
        console.log(x, y, z);
        card.position.x = x;
        card.position.y = y;
        card.position.z = z;
    }

    events.push(new PutCardForward(card, card_id));

    data.actions.forEach(action => {
        if (action.dice_roll > 0) {
            events.push(new ThrowDiceEvent(getPlayerById(action.player_dice_id), action.dice_roll));
        }

        action.targets.forEach(target => {
            const targetedPlayer = getPlayerById(target.player_id);

            switch (target.action.type) {
                case ActionTypeDTO.ATTACK:
                    events.push(new DamagePlayerEvent(targetedPlayer, target.action.amount));
                    break;
                case ActionTypeDTO.HEAL:
                    events.push(new HealPlayerEvent(targetedPlayer, target.action.amount));
                    break;
                case ActionTypeDTO.DRAW:
                    target.action.cards.forEach(card_id => {
                        events.push(new DrawCardEvent(targetedPlayer, card_id));
                    });
                    break;
                default:
                    console.log(`No event defined for \"${target.action.type}\"`);
                    break;
            }
        });
    });

    events.push(new PutCardInPile(player, card));

    eventMgr.pushEvents(events);
}

/** @param {DrawCardResponse} data  */
export function onDrawCardEvent(data) {
    const player = getPlayerById(data.player_id);

    console.log(player, data.card_id);

    eventMgr.pushEvent(new DrawCardEvent(player, data.card_id));
}

/** @param {ChangeTurnResponse} data  */
export function onChangeTurnEvent(data) {
    // updateCurrentPlayerTurn(getPlayerById(data.player_id));
    eventMgr.pushEvent(new ChangeTurnEvent(getPlayerById(data.player_id)));
}

/** @param {CollectDiscardCardsResponse} data  */
export function onCollectDiscardCardsEvent(data) {
    eventMgr.pushEvent(new CollectDiscardCardsEvent(data.cards_in_pile));
}

/** @param {GameEndResponse} data  */
export function onGameEndEvent(data) {
    eventMgr.pushEvent(new GameEndEvent(data.winner_id));
}


export function displayGameEndScreen(player_id) {
    winner_id = player_id;
    const winner = getPlayerById(winner_id);
    const win_text = winner != null ? `The game has ended. The winner is ${winner.name} !` : "The game has ended in a draw !";
    displayPopup(win_text, "Game End", "Next", () => {
        window.location.href = "index.html";
    });
}


export function setOpponentCardCount(id, count) {
    if (count < 0) { count = 0; }

    const opponent = OPPONENTS.get(id);

    if (opponent != null) {
        opponent.setCardCount(count);

    }
}


export function updateOpponentPositions() {
    let i = 0;

    OPPONENTS.forEach(opponent => {
        const { cx, cy, cz } = getOpponentPosition(i);
        opponent.position.set(cx, cy, cz);

        i++;
    });

}


export function getOpponentPosition(index) {
    const opponents_count = OPPONENTS.size;
    const space_between_opponents = 6;

    const cx = space_between_opponents*index + space_between_opponents / 2 - (space_between_opponents*opponents_count) / 2;
    const cy = 0.2;
    const cz = -4;

    return { cx, cy, cz };
}


/**
 * Used by ChangeTurnEvent to update the scene when changing player turn
 * @param {Player|null} who
 */
export function updateCurrentPlayerTurn(who, turn_end=0) {

    PLAYER.clearSelection();
    OPPONENTS.values().forEach(opponent => {
        opponent.clearSelection();
    });

    if (who == PLAYER && current_player_turn != PLAYER) {
        
    } else if (who != PLAYER && current_player_turn == PLAYER) {
        
        
        cardTooltip.visible = false;
    }

    current_player_turn = who;

}


export function getPlayerById(player_id) {
    if (player_id == session_info.id)
        return PLAYER;
    else
        return OPPONENTS.get(player_id);
}


export function getIdByPlayer(player) {
    if (player == PLAYER) {
        return session_info.id;
    } else {
        for (const [id, opp] of OPPONENTS.entries()) {
            if (opp == player)
                return id;
        }
        return undefined;
    }
}



function renderSceneView() {
    requestAnimationFrame(renderSceneView);
    renderer.render( scene, camera );
    labelRenderer.render( scene, camera );
    if (eventMgr)
        document.getElementById("event-counter").textContent = eventMgr.queueCount;
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );
    labelRenderer.setSize( window.innerWidth, window.innerHeight );
}


function onPointerMove( event ) {

    if (current_player_turn == PLAYER && !eventMgr.isWaitingForEvents()) {

        pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

        raycaster.setFromCamera( pointer, camera );

        const intersects = raycaster.intersectObjects( PLAYER.cards, false );

        if ( intersects.length > 0 ) {

            const card = intersects[ 0 ].object;

            // hover

            if (cardTooltip.card != card)
                cardTooltip.update(card);

            cardTooltip.position.set(card.position.x, card.position.y, card.position.z-2.5);
            cardTooltip.visible = true;

        } else {
            cardTooltip.visible = false;
        }
    }
}


function onPointerDown( event ) {

    if (current_player_turn == PLAYER && !eventMgr.isWaitingForEvents()) {

        pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

        raycaster.setFromCamera( pointer, camera );

        const intersects = raycaster.intersectObjects( PLAYER.cards /* scene.children */, false );

        if ( intersects.length > 0 ) {

            const card = intersects[ 0 ].object;

            if (event.button == 0) {
                PLAYER.toggleCardSelection(PLAYER.cards.indexOf(card));
            }

        } else if (PLAYER.selected_card != null) {  // choose target, TODO handle multiple targets

            if (PLAYER.selected_card.info.heal > 0 && PLAYER.selected_card.info.attack <= 0) {  // only heal
                const player_intersects = raycaster.intersectObject(PLAYER, false);
                
                // if player is selected, play FOOD card
                if (player_intersects.length > 0) {
                    const card_index = PLAYER.cards.indexOf(PLAYER.selected_card);
                    eventMgr.pushEvent(new ChangeTurnEvent(null));
                    serverConnexion.send_play_card_action(card_index, [] /* no target for food */);
                }
            } else {    /* must select target */
                const opponents = [];
                OPPONENTS.values().forEach(opponent => {
                    if (opponent.health > 0)
                        opponents.push(opponent);
                });

                const opponent_intersects = raycaster.intersectObjects(opponents, false);

                if (opponent_intersects.length > 0) {
                    const opponent = opponent_intersects[0].object;
                    const card_index = PLAYER.cards.indexOf(PLAYER.selected_card);

                    eventMgr.pushEvent(new ChangeTurnEvent(null));
                    serverConnexion.send_play_card_action(card_index, [getIdByPlayer(opponent)]);
                }
            }
        }
    }

}
