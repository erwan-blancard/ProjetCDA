import * as THREE from 'three';
import { Card, CardPile, getCardTexturePathById, OpponentCard } from './cards';
import { ServerConnexion } from './server/server_connection';
import { Opponent, Player } from './player';
import { PlayerUI } from './ui/player_ui';
import { CSS2DRenderer } from 'three-stdlib';
import { degToRad } from 'three/src/math/MathUtils';

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
export let player;
/** @type {Map<number, Opponent>} */
export const opponents = new Map();

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


export function initGame() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color().setHex(0xf0f0f0);
    camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize( window.innerWidth, window.innerHeight );
    // renderer.setPixelRatio( 2.0 );
    renderer.setPixelRatio( window.devicePixelRatio );
    document.body.appendChild( renderer.domElement );

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

    player = new Player(scene);
    player.position.set(0, 2, 5);
    const playerUI = new PlayerUI(player);
    playerUI.position.set(-4, 0, 0);

    const card1 = new Card(getCardTexturePathById(1));
    scene.add(card1);
    player.cards.push(card1);

    player.updateCardPositions();

    // test
    for (let i = 0; i < 5; i++) {
        const op = new Opponent(scene, 3);
        const ui = new PlayerUI(op);
        ui.position.set(-1, 2, 0);
        op.updateCardPositions();
        opponents.set(i, op);
    }

    // test
    setOpponentCardCount(3, 1);

    updateOpponentPositions();

    // Interactions

    renderer.domElement.addEventListener( 'pointermove', onPointerMove );
    renderer.domElement.addEventListener( 'pointerdown', onPointerDown );

    window.addEventListener( 'resize', onWindowResize );

    const gridHelper = new THREE.GridHelper(10, 10);
    scene.add(gridHelper);

    renderSceneView();

    serverConnexion = new ServerConnexion();

    // connect server events
    serverConnexion.addEventListener("connectionchange", ev => {
        console.log("Connection changed, status:", ev.detail.status);
    })
    serverConnexion.addEventListener("authchange", ev => {
        console.log("Auth status:", ev.detail.status);
    })
    serverConnexion.addEventListener("gameupdate", upd => {
        onServerUpdate(upd.detail);
    })
    serverConnexion.addEventListener("chatmessage", ev => {
        console.log("Chat message received:", ev.detail.msg);
    })
    serverConnexion.addEventListener("sessioninfo", ev => {
        onSessionInfoReceived(ev.detail);
    })
}


export function connectToServer(wsUri, token) {
    serverConnexion.connect(wsUri, token);
}


export function onServerUpdate(upd_data) {
    console.log("Game Update:", upd_data);

    try {
        const opponent_id = opponents[i]["player_id"];

        const current_player_turn = upd_data["current_player_turn"];
        const current_player_turn_end = upd_data["current_player_turn_end"];

        const health = upd_data["health"];
        const cards = upd_data["cards"];
        const discard_cards = upd_data["discard_cards"];

        // opponents
        const opponents = upd_data["opponents"];
        const seen_opponents = [];

        for (let i = 0; i < opponents.length; i++) {
            const opponent_id = opponents[i]["player_id"];
            seen_opponents.push(opponent_id);
            const health = opponents[i]["health"];
            const cards = opponents[i]["card_count"];
            const discard_cards = opponents[i]["discard_cards"];
        }
    } catch (e) {
        console.log("Exception when handling game update data:", e);
    }
}


// setup expected player count, names, and identifiers
export function onSessionInfoReceived(info) {
    const my_id = info.id;
    let my_profile = null;
    const opponents_profile = [];

    const IDs = new Set();

    for (let i = 0; i < info.players.length; i++) {
        const profile = info.players[i];

        if (IDs.has(profile.id)) {
            throw new Error("Duplicate player id found when parsing session info !");
        }

        IDs.add(profile.id);

        if (profile.id == my_id) {
            my_profile = profile;
        } else {
            opponents_profile.push(profile);
        }
    }

    if (my_profile == null) {
        throw new Error("Player profile not in array !");
    }

    if (opponents_profile.length < 1) {
        throw new Error("Not enought opponents !");
    }

    opponents_profile.forEach(profile => {
        const opponent = new Opponent(scene);
        opponent.name = profile.name;
        opponents.set(profile.id, opponent);
    });

    session_info = info;
}


function setOpponentCardCount(id, count) {
    if (count < 0) { count = 0; }

    const opponent = opponents.get(id);

    if (opponent != null) {
        opponent.setCardCount(count);

    }
}


function updateOpponentPositions() {
    let i = 0;

    opponents.forEach(opponent => {
        const { cx, cy, cz } = getOpponentPosition(i);
        opponent.position.set(cx, cy, cz);

        // opponent.lookAt(new THREE.Vector3(0, 10, 3));

        i++;
    });

}


function getOpponentPosition(index) {
    const opponents_count = opponents.size;
    const space_between_opponents = 4;

    const cx = space_between_opponents*index + space_between_opponents / 2 - (space_between_opponents*opponents_count) / 2;
    const cy = 2;
    const cz = -3;

    return { cx, cy, cz };
}



function renderSceneView() {
    requestAnimationFrame(renderSceneView);
    renderer.render( scene, camera );
    labelRenderer.render( scene, camera );
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );
    labelRenderer.setSize( window.innerWidth, window.innerHeight );
}

function onPointerMove( event ) {

    pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

    raycaster.setFromCamera( pointer, camera );

    const intersects = raycaster.intersectObjects( player.cards, false );

    if ( intersects.length > 0 ) {

        const intersect = intersects[ 0 ];

        // hover
    }
}


function onPointerDown( event ) {

    pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

    // console.log(pointer);

    raycaster.setFromCamera( pointer, camera );

    const intersects = raycaster.intersectObjects( player.cards /* scene.children */, false );
    // console.log(intersects.length);

    if ( intersects.length > 0 ) {

        const card = intersects[ 0 ].object;

        // interact
        // card.flipCard();
        console.log(event.button);

        switch (event.button) {
            case 0:
                card.flipCard();
                break;
            case 1:
                card.startSwingLoop();
                break;
        
            default:
                card.stopSwingLoop();
                card.quaternion.copy(camera.quaternion);    // card face camera
                break;
        }
    } else {
        const pile_intersects = raycaster.intersectObject( cardPile, false );

        if (pile_intersects.length > 0) {
            if (cardPile.count > 0) {
                console.log("draw card");
                const new_card = cardPile.drawCard();
                new_card.position.set(-10 + player.cards.length * 0.5, 2, -2);
                scene.add(new_card);
                player.cards.push(new_card);
            } else {
                console.log("no more cards");
            }
            
        }
    }

}
