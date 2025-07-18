import * as THREE from 'three';
import Nebula, { Emitter, Rate, Span, Position, Mass, Radius, Life, Body, Vector3D, RadialVelocity, Color, Alpha, Scale } from 'three-nebula';

export function initMenuParticles() {
  const container = document.getElementById('particles-menu-container');
  if (!container) return;

  // Création de la scène
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(75, container.offsetWidth / container.offsetHeight, 0.1, 1000);
  camera.position.z = 100;

  const renderer = new THREE.WebGLRenderer({ alpha: true });
  renderer.setClearColor(0x000000, 0); // fond transparent
  renderer.setSize(container.offsetWidth, container.offsetHeight);
  container.appendChild(renderer.domElement);

  // Système de particules Nebula
  const nebula = new Nebula(scene, THREE);
  const emitter = new Emitter();
  emitter
    .setRate(new Rate(new Span(10, 20), new Span(0.1, 0.25)))
    .addInitializers([
      new Position(new Vector3D(0, 0, 0)),
      new Mass(1),
      new Radius(10, 20),
      new Life(2, 4),
      new Body(new THREE.SpriteMaterial({ color: 0x00aaff })),
      new RadialVelocity(30, new Vector3D(0, 1, 0), 180)
    ])
    .addBehaviours([
      new Color('white', 'blue'),
      new Alpha(1, 0),
      new Scale(1, 2)
    ]);
  nebula.addEmitter(emitter);

  // Animation loop
  function animate() {
    requestAnimationFrame(animate);
    nebula.update();
    renderer.render(scene, camera);
  }
  animate();

  // Responsive
  window.addEventListener('resize', () => {
    camera.aspect = container.offsetWidth / container.offsetHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(container.offsetWidth, container.offsetHeight);
  });
} 