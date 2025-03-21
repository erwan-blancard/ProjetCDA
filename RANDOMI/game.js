import * as THREE from 'three';

// Initialisation de la scène
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Ajout d'une lumière directionnelle
const light = new THREE.DirectionalLight(0xffffff, 1);
light.position.set(5, 10, 5);
scene.add(light);

// Création de l'arène circulaire
const arenaGeometry = new THREE.CircleGeometry(6, 64);
const arenaMaterial = new THREE.MeshStandardMaterial({ color: 0x008000 });
const arena = new THREE.Mesh(arenaGeometry, arenaMaterial);
arena.rotation.x = -Math.PI / 2;
scene.add(arena);

// Position de la caméra
camera.position.set(0, 8, 10);
camera.lookAt(0, 0, 0);

// Fonction pour créer une carte
function createCard(x, z, texturePath) {
    const cardGeometry = new THREE.PlaneGeometry(1, 1.5);
    const cardTexture = new THREE.TextureLoader().load(texturePath);
    const cardMaterial = new THREE.MeshStandardMaterial({ map: cardTexture });
    const card = new THREE.Mesh(cardGeometry, cardMaterial);
    card.position.set(x, 0.2, z);
    card.rotation.x = -Math.PI / 2;
    scene.add(card);
    return card;
}

// Création de la pioche au centre
const drawPile = createCard(0, 0, 'assets/deck.jpg');

// Création des piles de défausse pour chaque joueur
const discardPiles = [];
const playerPositions = [
    { x: 5, z: 0 },
    { x: 3.5, z: 3.5 },
    { x: 0, z: 5 },
    { x: -3.5, z: 3.5 },
    { x: -5, z: 0 },
    { x: -3.5, z: -3.5 }
];

for (let i = 0; i < 6; i++) {
    discardPiles.push(createCard(playerPositions[i].x, playerPositions[i].z, 'assets/discard.jpg'));
}

// Création des cartes en main pour chaque joueur
const hands = [];
for (let i = 0; i < 6; i++) {
    hands.push([
        createCard(playerPositions[i].x - 1, playerPositions[i].z - 1, 'assets/card1.jpg'),
        createCard(playerPositions[i].x, playerPositions[i].z - 1, 'assets/card2.jpg'),
        createCard(playerPositions[i].x + 1, playerPositions[i].z - 1, 'assets/card3.jpg')
    ]);
}

// Bouton pour terminer le tour
const endTurnButton = document.createElement('button');
endTurnButton.innerText = 'Fin de tour';
endTurnButton.style.position = 'absolute';
endTurnButton.style.bottom = '20px';
endTurnButton.style.left = '50%';
endTurnButton.style.transform = 'translateX(-50%)';
endTurnButton.style.padding = '10px 20px';
document.body.appendChild(endTurnButton);

endTurnButton.addEventListener('click', () => {
    console.log('Tour terminé !');
    socket.send(JSON.stringify({ action: 'end_turn' }));
});

// Connexion WebSocket
const socket = new WebSocket('ws://127.0.0.1:3030/ws');
socket.onopen = () => {
    console.log('Connecté au serveur WebSocket');
};

socket.onmessage = (event) => {
    console.log('Message reçu:', event.data);
};

// Animation de rendu
function animate() {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
}
animate();
