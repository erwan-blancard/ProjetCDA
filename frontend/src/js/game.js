import * as THREE from 'three';
import { Card, CardPile, getCardTexturePathById } from './cards';

export let scene, camera, renderer;

export let raycaster = new THREE.Raycaster();
export let pointer = new THREE.Vector2();
export const player_cards = [];
export let cardPile;


export function initGame() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color().setHex(0xf0f0f0);
    camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize( window.innerWidth, window.innerHeight );
    renderer.setPixelRatio( window.devicePixelRatio );
    document.body.appendChild( renderer.domElement );

    camera.position.set(0, 8, 8);
    camera.lookAt(new THREE.Vector3(0, 2, 2));

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

    const card1 = new Card(getCardTexturePathById(1));
    card1.position.set(-2, 6, 5);
    scene.add(card1);
    player_cards.push(card1);

    // Interactions

    document.addEventListener( 'pointermove', onPointerMove );
    document.addEventListener( 'pointerdown', onPointerDown );

    window.addEventListener( 'resize', onWindowResize );

    const gridHelper = new THREE.GridHelper(10, 10);
    scene.add(gridHelper);

    // const geometry = new THREE.BoxGeometry( 1, 1, 1 );
    // const material = new THREE.MeshBasicMaterial( { color: 0x00ff00, opacity: 0.5 } );
    // const cube = new THREE.Mesh( geometry, material );
    // scene.add( cube );

    renderer.setAnimationLoop( () => {
        // cube.rotateY(THREE.MathUtils.degToRad(0.1));
        renderSceneView();
    } );
}

function renderSceneView() {
    renderer.render( scene, camera );
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );
}

function onPointerMove( event ) {

    pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

    raycaster.setFromCamera( pointer, camera );

    const intersects = raycaster.intersectObjects( player_cards, false );

    if ( intersects.length > 0 ) {

        const intersect = intersects[ 0 ];

        // hover
    }
}


function onPointerDown( event ) {

    pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

    // console.log(pointer);

    raycaster.setFromCamera( pointer, camera );

    const intersects = raycaster.intersectObjects( player_cards /* scene.children */, false );
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
                card.quaternion.copy(camera.quaternion);
                break;
        }
    } else {
        const pile_intersects = raycaster.intersectObject( cardPile, false );

        if (pile_intersects.length > 0) {
            if (cardPile.count > 0) {
                console.log("draw card");
                const new_card = cardPile.drawCard();
                new_card.position.set(-10 + player_cards.length * 0.5, 2, -2);
                scene.add(new_card);
                player_cards.push(new_card);
            } else {
                console.log("no more cards");
            }
            
        }
    }

}
