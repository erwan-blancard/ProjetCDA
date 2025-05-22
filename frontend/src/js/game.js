import * as THREE from 'three';
import { Card, CardPile, getCardTexturePathById } from './cards.js';
import { playCard, drawCard } from './main.js';

export let scene, camera, renderer;
export let raycaster = new THREE.Raycaster();
export let pointer   = new THREE.Vector2();
export const player_cards = [];
export let cardPile;

export function initGame() {
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0);
  camera = new THREE.PerspectiveCamera(75, window.innerWidth/window.innerHeight, 0.1, 1000);
  camera.position.set(0,8,8);
  camera.lookAt(0,2,2);

  renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.getElementById('game-container').appendChild(renderer.domElement);

  scene.add(new THREE.AmbientLight(0x606060, 3));
  const dir = new THREE.DirectionalLight(0xffffff, 3);
  dir.position.set(1,0.75,0.5).normalize();
  scene.add(dir);

  const boardTex = new THREE.TextureLoader().load('assets/board.jpg');
  const boardMat = new THREE.MeshStandardMaterial({ map: boardTex });
  const boardGeo = new THREE.PlaneGeometry(10,10);
  const board = new THREE.Mesh(boardGeo, boardMat);
  board.rotation.x = -Math.PI/2;
  scene.add(board);

  cardPile = new CardPile();
  scene.add(cardPile);

  scene.add(new THREE.GridHelper(10,10));

  document.addEventListener('pointermove', onPointerMove);
  document.addEventListener('pointerdown', onPointerDown);
  window.addEventListener('resize', onWindowResize);

  renderer.setAnimationLoop(() => renderer.render(scene, camera));
}

function onWindowResize() {
  camera.aspect = window.innerWidth/window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
}

function onPointerMove(event) {
  pointer.set(
    (event.clientX/window.innerWidth)*2 - 1,
    -(event.clientY/window.innerHeight)*2 + 1
  );
  raycaster.setFromCamera(pointer, camera);
}

function onPointerDown(event) {
  onPointerMove(event);
  const hitCards = raycaster.intersectObjects(player_cards, false);
  if (hitCards.length > 0) {
    const card = hitCards[0].object;
    playCard(card.userData.id);
  } else {
    const hitPile = raycaster.intersectObject(cardPile, false);
    if (hitPile.length > 0 && cardPile.count > 0) {
      drawCard();
    }
  }
}

export function onOpponentPlay(playerId, cardId) {
  console.log(`${playerId} joue la carte ${cardId}`);
}

export function onOpponentDraw(playerId, cardId) {
  console.log(`${playerId} pioche la carte ${cardId}`);
}

export function setHand(hand) {
  player_cards.forEach(c => scene.remove(c));
  player_cards.length = 0;

  hand.forEach((cid, i) => {
    const c = new Card(getCardTexturePathById(cid));
    c.userData.id = cid;
    c.position.set(-2 + i*1.2, 6, 5);
    scene.add(c);
    player_cards.push(c);
  });
}
