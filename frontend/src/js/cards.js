import * as THREE from 'three';
import gsap, { Power1, Power2 } from 'gsap';
import { clamp, randInt } from 'three/src/math/MathUtils.js';


const textureLoader = new THREE.TextureLoader();
const cardGeo = new THREE.BoxGeometry(1, 1.5, 0.001);
const cardCover = textureLoader.load("assets/randomi_verso.jpg");
cardCover.colorSpace = THREE.SRGBColorSpace;


export function getCardTexturePathById(id) {
    const number = id.toString().padStart(4, "0");
    return `assets/randomi_recto_cards_page-${number}.jpg`;
}


export class CardPile extends THREE.Mesh {
    static max_visible_cards = 25;
    static card_thickness = 0.05;
    #count;

    constructor() {
        const cardPileGeo = new THREE.BoxGeometry(1, CardPile.card_thickness * CardPile.max_visible_cards, 1.5);

        const mats = [
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial({map: cardCover}),
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial()
        ]

        super(cardPileGeo, mats);
        this.#count = CardPile.max_visible_cards;
        this.#updatePile();
    }

    get count() {
        return this.#count;
    }

    set count(value) {
        this.#count = value;
        this.#updatePile();
    }

    drawCard() {
        const card_id = randInt(1, 100);
        this.count -= 1;
        return new Card(getCardTexturePathById(card_id));
    }

    #updatePile() {
        // set pile position based on remaining cards
        const height = CardPile.card_thickness * clamp(this.#count, 0, CardPile.max_visible_cards);
        this.position.y = -(CardPile.card_thickness * CardPile.max_visible_cards) / 2 + height;

        this.visible = this.#count > 0;

        console.log(`Height: ${height}, Count: ${this.#count}, Visible: ${this.visible}`);
    }

}


export class Card extends THREE.Mesh {
    flipped = false;
    swingTimeline = gsap.timeline();

    constructor(image, ) {
        const tex = textureLoader.load(image);
        tex.colorSpace = THREE.SRGBColorSpace;

        const mats = [
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial(),
            new THREE.MeshBasicMaterial({map: tex}),
            new THREE.MeshBasicMaterial({map: cardCover})
        ]

        super(cardGeo, mats);
    }

    flipCard() {
        const tl = gsap.timeline();
        tl.to(this.rotation, { y: (this.flipped ? 0 : -Math.PI), duration: 0.5 } );
        this.flipped = !this.flipped;
    }

    startSwingLoop() {
        const angle = THREE.MathUtils.degToRad(4);
        this.swingTimeline.fromTo(this.rotation, { z: -angle }, { z: angle, repeat: -1, duration: 1, yoyo: true, ease: Power1.easeInOut } );
    }

    stopSwingLoop() {
        this.swingTimeline.clear();
        this.rotation.z = 0;
    }

}
