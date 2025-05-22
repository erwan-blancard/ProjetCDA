// frontend/tests/game.test.js
import * as THREE from 'three';
import { createCard } from '../src/game.js';

describe('createCard()', () => {
  let scene, camera, renderer;
  beforeAll(() => {
    // On ajoute une scène fictive pour Three.js
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera();
    renderer = new THREE.WebGLRenderer();
    document.body.innerHTML = '<div id="game-container"></div>';
  });

  test('retourne un Mesh de type PlaneGeometry', () => {
    const card = createCard(1, 2, 'assets/card1.jpg');
    expect(card).toBeInstanceOf(THREE.Mesh);
    expect(card.geometry).toBeInstanceOf(THREE.PlaneGeometry);
    expect(card.position.x).toBe(1);
    expect(card.position.z).toBe(2);
    // Cleanup
    scene.remove(card);
  });
});
