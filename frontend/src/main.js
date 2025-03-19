function getCookie(name) {
    const match = document.cookie
                  .match(new RegExp("(^| )" + name + "=([^;]+)"));
    if (match) return match[2];
    return null;
}

if (!getCookie("token")) {
    alert("Not logged in!");
    window.location.href = "/login.html";
}

import * as THREE from 'three';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

const renderer = new THREE.WebGLRenderer();
renderer.setSize( window.innerWidth, window.innerHeight );
document.body.appendChild( renderer.domElement );


const geometry = new THREE.BoxGeometry( 1, 1, 1 );
const material = new THREE.MeshBasicMaterial( { color: 0x00ff00 } );
const cube = new THREE.Mesh( geometry, material );
scene.add( cube );

camera.position.z = 5;

cube.rotateX(THREE.MathUtils.degToRad(22.5));

function animate() {
    cube.rotateY(THREE.MathUtils.degToRad(0.1));
	renderer.render( scene, camera );
}
renderer.setAnimationLoop( animate );
