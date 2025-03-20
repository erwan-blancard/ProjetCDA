import * as THREE from 'three';

export let scene, camera, renderer;

export let raycaster = new THREE.Raycaster();
export let pointer = new THREE.Vector2();
export const objects = [];

function renderSceneView() {
    renderer.render( scene, camera );
}


export function initGame() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color().setHex(0xf0f0f0);
    camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

    renderer = new THREE.WebGLRenderer();
    renderer.setSize( window.innerWidth, window.innerHeight );
    renderer.setPixelRatio( window.devicePixelRatio );
    document.body.appendChild( renderer.domElement );

    camera.position.z = 10;

    // lights

    const ambientLight = new THREE.AmbientLight( 0x606060, 3 );
    scene.add( ambientLight );

    const directionalLight = new THREE.DirectionalLight( 0xffffff, 3 );
    directionalLight.position.set( 1, 0.75, 0.5 ).normalize();
    scene.add( directionalLight );

    // Interactions

    document.addEventListener( 'pointermove', onPointerMove );
    document.addEventListener( 'pointerdown', onPointerDown );

    window.addEventListener( 'resize', onWindowResize );

    const geometry = new THREE.BoxGeometry( 1, 1, 1 );
    const material = new THREE.MeshBasicMaterial( { color: 0x00ff00, opacity: 0.5 } );
    const cube = new THREE.Mesh( geometry, material );
    scene.add( cube );
    cube.rotateX(THREE.MathUtils.degToRad(22.5));

    objects.push(cube);

    renderer.setAnimationLoop( () => {
        cube.rotateY(THREE.MathUtils.degToRad(0.1));
        renderSceneView();
    } );
}

function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );
}

function onPointerMove( event ) {

    pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

    raycaster.setFromCamera( pointer, camera );

    const intersects = raycaster.intersectObjects( objects, false );

    if ( intersects.length > 0 ) {

        const intersect = intersects[ 0 ];

        // hover
    }
}


function onPointerDown( event ) {

    pointer.set( ( event.clientX / window.innerWidth ) * 2 - 1, - ( event.clientY / window.innerHeight ) * 2 + 1 );

    console.log(pointer);

    raycaster.setFromCamera( pointer, camera );

    const intersects = raycaster.intersectObjects( objects /* scene.children */, false );
    console.log(intersects.length);

    if ( intersects.length > 0 ) {

        const intersect = intersects[ 0 ];

        // interact
    }

}
