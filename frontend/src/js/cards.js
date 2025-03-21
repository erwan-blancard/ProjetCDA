import * as THREE from 'three';
import gsap, { Power1, Power2 } from 'gsap';


const textureLoader = new THREE.TextureLoader();
const cardGeo = new THREE.BoxGeometry(1, 1.5, 0.001);
const cardCover = textureLoader.load("assets/randomi_verso.jpg");
cardCover.colorSpace = THREE.SRGBColorSpace;


export class Card extends THREE.Mesh {
    flipped = false;
    swingTimeline = gsap.timeline({ paused: true });

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
        
        const angle = THREE.MathUtils.degToRad(4);
        this.swingTimeline.fromTo(this.rotation, { z: -angle }, { z: angle, repeat: -1, duration: 1, yoyo: true, ease: Power1.easeInOut } );
    }

    flipCard() {
        const tl = gsap.timeline();
        tl.to(this.rotation, { y: this.flipped ? -Math.PI : 0, duration: 0.5 } );
        this.flipped = !this.flipped;
    }

    startSwingLoop() {
        this.swingTimeline.restart();
    }

    stopSwingLoop() {
        this.swingTimeline.pause();
    }

}
