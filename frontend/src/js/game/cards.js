import * as THREE from 'three';
import gsap, { Power1, Power2 } from 'gsap';
import { clamp, randInt } from 'three/src/math/MathUtils.js';
import { CARD_DATABASE, CardInfo } from './database';


const textureLoader = new THREE.TextureLoader();
const cardGeo = new THREE.BoxGeometry(1, 1.5, 0.001);
export const CARD_COVER_PATH = "assets/randomi_verso.jpg";
const cardCover = textureLoader.load(CARD_COVER_PATH);
cardCover.colorSpace = THREE.SRGBColorSpace;


export function newCardMat(front, cover=cardCover) {
    const mats = [
        new THREE.MeshBasicMaterial(),
        new THREE.MeshBasicMaterial(),
        new THREE.MeshBasicMaterial(),
        new THREE.MeshBasicMaterial(),
        new THREE.MeshBasicMaterial({map: front}),
        new THREE.MeshBasicMaterial({map: cover}),
    ];
    return mats;
}


export const COVER_FRONTBACK_MAT = newCardMat(cardCover);


export function getCardTexturePathById(id) {
    if (id < 0) return CARD_COVER_PATH;
    const number = (id + 1).toString().padStart(4, "0");
    return `assets/randomi_recto_cards_page-${number}.jpg`;
}


export class CardPile extends THREE.Mesh {
    static max_visible_cards = 25;
    static card_thickness = 0.04;
    #count;

    constructor(card_count = 100) {
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
        this.#count = card_count;
        this.#updatePile();
    }

    get count() {
        return this.#count;
    }

    set count(value) {
        this.#count = value;
        this.#updatePile();
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
    card_id = -1;
    /** @type {CardInfo | null} */
    info = null;
    flipped = false;
    swingTimeline = gsap.timeline();

    constructor(card_id) {
        const image = getCardTexturePathById(card_id);
        const tex = textureLoader.load(image);
        tex.colorSpace = THREE.SRGBColorSpace;

        super(cardGeo, newCardMat(tex));
        this.card_id = card_id;
        this.info = CARD_DATABASE.get(this.card_id);
        this.rotateX(THREE.MathUtils.degToRad(-90));
    }

    /** transition smoothly to next position */
    goto(x, y, z, duration=0.4) {
        const tl = gsap.timeline();
        tl.to(this.position, { x, y, z, duration });
    }

    flipCard(instant=false) {
        const tl = gsap.timeline();
        tl.to(this.rotation, { y: (this.flipped ? 0 : -Math.PI), duration: (instant ? 0.0 : 0.25) } );
        this.flipped = !this.flipped;
    }

    startSwingLoop() {
        this.swingTimeline.clear();
        const angle = THREE.MathUtils.degToRad(4);
        this.swingTimeline.fromTo(this.rotation, { z: -angle }, { z: angle, repeat: -1, duration: 1, yoyo: true, ease: Power1.easeInOut } );
    }

    stopSwingLoop() {
        this.swingTimeline.clear();
        this.rotation.z = 0;
    }

}


export class OpponentCard extends Card {

    constructor() {
        super(-1);
        this.flipCard(true);
    }

    displayCardAsFront(id) {
        const image = getCardTexturePathById(id);
        const tex = textureLoader.load(image);
        tex.colorSpace = THREE.SRGBColorSpace;
        
        const mats = newCardMat(tex);
        this.material = mats;
    }

    displayCoverAsFront() {
        const mats = COVER_FRONTBACK_MAT;
        this.material = mats;
    }

}
