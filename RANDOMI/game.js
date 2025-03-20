import * as THREE from 'three';

// Initialisation de la scène
const scene = new THREE.Scene();
scene.background = new THREE.Color().setHex(0xf0f0f0);
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Ajout d'une lumière directionnelle
const light = new THREE.DirectionalLight(0xffffff, 1);
light.position.set(5, 10, 5);
scene.add(light);

// Création du plateau avec texture
const boardTexture = new THREE.TextureLoader().load('assets/board.jpg');
const boardMaterial = new THREE.MeshStandardMaterial({ map: boardTexture });
const boardGeometry = new THREE.PlaneGeometry(10, 10);
const board = new THREE.Mesh(boardGeometry, boardMaterial);
board.rotation.x = -Math.PI / 2;
scene.add(board);

// Position de la caméra
camera.position.set(0, 6, 8);
camera.lookAt(0, 0, 0);

// Création des cartes avec animations et effets
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

// Connexion WebSocket
const socket = new WebSocket('ws://127.0.0.1:3030/ws');
socket.onopen = () => {
    console.log('Connecté au serveur WebSocket');
};

socket.onmessage = (event) => {
    console.log('Message reçu:', event.data);
};

// Ajout de cartes avec différentes textures
const cards = [
    createCard(-2, -2, 'assets1.jpg'),
    createCard(0, -2, 'assets2.jpg'),
    createCard(2, -2, 'assets3.jpg')
];

// Interaction : cliquer sur une carte et envoyer l'action au serveur
window.addEventListener('click', (event) => {
    const raycaster = new THREE.Raycaster();
    const mouse = new THREE.Vector2();
    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;
    raycaster.setFromCamera(mouse, camera);
    
    const intersects = raycaster.intersectObjects(cards);
    if (intersects.length > 0) {
        const selectedCard = intersects[0].object;
        selectedCard.position.y += 0.5; // Simule la sélection
        console.log('Carte sélectionnée !');
        socket.send(JSON.stringify({ action: 'play_card', card: selectedCard.uuid }));
    }
});

// Animation de rendu
function animate() {
    renderer.render(scene, camera);
}

renderer.setAnimationLoop( animate );
