import * as THREE from 'three';
import { Card, CardPile, getCardTexturePathById, OpponentCard } from './cards';
import { ServerConnexion } from '../server/server_connection';
import { Opponent, Player } from './player';
import { PlayerUI } from '../ui/player_ui';
import { CSS2DRenderer, OrbitControls } from 'three-stdlib';
import { degToRad } from 'three/src/math/MathUtils';
import { CardTooltip } from '../ui/card_tooltip';
import { ActionTypeDTO, ChangeTurnResponse, CollectDiscardCardsResponse, DrawCardResponse, GameEndResponse, GameStatusResponse, PlayCardResponse, PlayerBuffStatusResponse, SessionInfoResponse } from '../server/dto';
import { EventMgr } from './events/event_mgr';
import { ChangeTurnEvent, DamagePlayerEvent, DrawCardEvent, GameUpdateEvent, HealPlayerEvent, PutCardInPile, PutCardForward, ThrowDiceEvent, CollectDiscardCardsEvent, GameEndEvent, DiscardCardEvent, PlayerBuffsUpdateEvent } from './events/events';
import { displayPopup } from '../ui/popup';
import { CardKind, TargetType } from './collection';
import { Dice } from '../ui/dice';
import gsap, { Power1 } from 'gsap';

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

export let currentPlayerTurn;
/** @type {number} */
export let currentPlayerTurnEnd = 0;

/** @type {HTMLElement | null} */
let turnTimer = null;

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
export let sessionInfo;

/** @type {SelectHintBox | null} */
export let selectHintBox;

/** @type {CardPile | null} */
export let cardPile;

/** @type {ServerConnexion | null} */
export let serverConnexion;

/** @type {CardTooltip | null} */
export let cardTooltip;

/** @type {Dice | null} */
export let dice;

/** @type {EventMgr | null} */
export let eventMgr;

/** @type {number | null} */
export let winnerId = null;


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

    // target all hint
    selectHintBox = new SelectHintBox(boardGeometry);
    selectHintBox.rotation.x = -Math.PI / 2;
    selectHintBox.position.set(0, 9, 0);
    scene.add(selectHintBox);

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

    dice = new Dice(scene);

    renderSceneView();

    eventMgr = new EventMgr();

    turnTimer = document.getElementById("turn-timer");

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
    serverConnexion.addEventListener("playerbuffstatus", ev => {
        onPlayerBuffStatusEvent(ev.detail);
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

    sessionInfo = info;

    updateOpponentPositions();

    OPPONENTS.forEach(opponent => {
        opponent.updateHandCardPositions();
    });
}

/** @param {PlayCardResponse} data  */
export function onPlayCardEvent(data) {
    updateCurrentPlayerTurn(null);  // hide timer and disable interactions

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
                case ActionTypeDTO.DISCARD:
                    // sort indexes in descending order
                    target.action.cards.sort((a, b) => a < b).forEach(card_index => {
                        events.push(new DiscardCardEvent(targetedPlayer, card_index));
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
    eventMgr.pushEvent(new ChangeTurnEvent(getPlayerById(data.player_id), data.turn_end));
}

/** @param {PlayerBuffStatusResponse} data  */
export function onPlayerBuffStatusEvent(data) {
    eventMgr.pushEvent(new PlayerBuffsUpdateEvent(getPlayerById(data.player_id), data.buffs));
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
    winnerId = player_id;
    const winner = getPlayerById(winnerId);
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

    stopSelectionGlow();

    PLAYER.clearSelection();
    OPPONENTS.values().forEach(opponent => {
        opponent.clearSelection();
    });

    if (who == PLAYER && currentPlayerTurn != PLAYER) {
        
    } else if (who != PLAYER && currentPlayerTurn == PLAYER) {
        
        
        cardTooltip.visible = false;
    }

    currentPlayerTurn = who;
    currentPlayerTurnEnd = turn_end;

}


export function getPlayerById(player_id) {
    if (player_id == sessionInfo.id)
        return PLAYER;
    else
        return OPPONENTS.get(player_id);
}


export function getIdByPlayer(player) {
    if (player == PLAYER) {
        return sessionInfo.id;
    } else {
        for (const [id, opp] of OPPONENTS.entries()) {
            if (opp == player)
                return id;
        }
        return undefined;
    }
}


function stopSelectionGlow() {
    PLAYER.stopGlowLoop();
    OPPONENTS.values().forEach(opponent => {
        opponent.stopGlowLoop();
    });
    selectHintBox.stopGlowLoop();
}


function updateTurnTimer() {
    const now = Math.ceil(Date.now() / 1000);   // in seconds
    const diff = currentPlayerTurnEnd - now;
    turnTimer.textContent = currentPlayerTurnEnd > 0 ? (diff + 1 >= 0 ? diff + 1 : 0) : "";
}

function renderSceneView() {
    requestAnimationFrame(renderSceneView);
    renderer.render( scene, camera );
    labelRenderer.render( scene, camera );
    if (eventMgr)
        document.getElementById("event-counter").textContent = eventMgr.queueCount;
    if (turnTimer)
        updateTurnTimer();
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );
    labelRenderer.setSize( window.innerWidth, window.innerHeight );
}


function onPointerMove( event ) {

    if (currentPlayerTurn == PLAYER && !eventMgr.isWaitingForEvents()) {

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

    if (currentPlayerTurn == PLAYER && !eventMgr.isWaitingForEvents()) {

        pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

        raycaster.setFromCamera( pointer, camera );

        const card_intersects = raycaster.intersectObjects( PLAYER.cards /* scene.children */, false );

        if ( card_intersects.length > 0 ) {

            const card = card_intersects[ 0 ].object;

            if (event.button == 0) {
                PLAYER.toggleCardSelection(PLAYER.cards.indexOf(card));

                stopSelectionGlow();

                if (PLAYER.selected_card != null) {
                    console.log(`SELECT CARD TARGET TYPE: ${PLAYER.selected_card.info.targets}`);
                    switch (PLAYER.selected_card.info.targets) {
                        case TargetType.SINGLE:
                        case TargetType.MULTIPLE:
                            OPPONENTS.values().forEach(opponent => {
                                opponent.startGlowLoop();
                            });
                            break;
                        case TargetType.SINGLE_AND_SELF:
                        case TargetType.MULTIPLE_AND_SELF:
                            PLAYER.startGlowLoop();
                            OPPONENTS.values().forEach(opponent => {
                                opponent.startGlowLoop();
                            });
                            break;
                        case TargetType.SELF:
                            PLAYER.startGlowLoop();
                            break;
                        case TargetType.ALL:
                        case TargetType.ALL_AND_SELF:
                            // TODO
                            selectHintBox.startGlowLoop();
                            break;
                    }

                }
            }

        } else {
            // choose target, TODO handle multiple targets
            handleTargetSelection();
        }
    }

}


function handleTargetSelection() {
    // raycaster is already setup

    if (PLAYER.selected_card != null) {
        
        switch (PLAYER.selected_card.info.targets) {
            // TODO handle MULTIPLE target type
            case TargetType.SINGLE:     // select opponent to play the card
            case TargetType.SINGLE_AND_SELF:
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
                break;
            case TargetType.ALL:    // select hint box to play the card
            case TargetType.ALL_AND_SELF:
                const select_hint_intersects = raycaster.intersectObject(selectHintBox, false);
            
                // if hint box is selected, play card
                if (select_hint_intersects.length > 0) {
                    const card_index = PLAYER.cards.indexOf(PLAYER.selected_card);
                    eventMgr.pushEvent(new ChangeTurnEvent(null));
                    serverConnexion.send_play_card_action(card_index, [] /* targets are filled by server */);
                }
                break;
            case TargetType.SELF:    // select self to play the card
                const player_intersects = raycaster.intersectObject(PLAYER, false);
            
                // if player is selected, play card
                if (player_intersects.length > 0) {
                    const card_index = PLAYER.cards.indexOf(PLAYER.selected_card);
                    eventMgr.pushEvent(new ChangeTurnEvent(null));
                    serverConnexion.send_play_card_action(card_index, [] /* no targets needed */);
                }
                break;
        }
    }
}


class SelectHintBox extends THREE.Mesh {
    /** @type {gsap.core.Timeline} */
    glowTl = gsap.timeline();

    constructor(geo) {
        const mat = new THREE.MeshBasicMaterial( { color: 0xffffff, opacity: 0.0, transparent: true } );
        super(geo, mat);
    }

    startGlowLoop() {
        this.glowTl.clear();
        this.material.opacity = 0.0;
        this.glowTl.to(this.material, { opacity: 0.3, duration: 0.5, repeat: -1, yoyo: true, ease: Power1.easeInOut });
    }

    stopGlowLoop() {
        this.glowTl.clear();
        this.material.opacity = 0.0;
    }
}
