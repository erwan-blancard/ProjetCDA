import { CSS2DObject } from "three-stdlib";
import {gsap, Power1} from "gsap";
import { randInt } from "three/src/math/MathUtils";


// dice faces are all visible on the svg
const DICE_SVG = await fetch("/assets/dice.svg").then(res => res.text());
const TARGET_APPEAR_WIDTH = "8em";


export class Dice extends CSS2DObject {
    wiggleTl = gsap.timeline();
    appearTl = gsap.timeline();
    diceSvg;
    playerNameLabel;

    constructor(scene) {
        const element = document.createElement("div");
        element.className = "dice";
        element.innerHTML = DICE_SVG;
        super(element);

        this.diceSvg = this.element.firstElementChild;
        this.playerNameLabel = document.createElement("p");
        this.playerNameLabel.className = "dice-player-name";
        this.playerNameLabel.style.visibility = "hidden";

        this.element.appendChild(this.playerNameLabel);

        this.diceSvg.style.width = "0em";
        this.setVisibleFace(6);

        scene.add(this);
    }

    setPlayerName(name) {
        this.playerNameLabel.textContent = name;
    }

    setVisibleFace(face) {
        for (let i = 1; i <= 6; i++) {
            const group = document.getElementById(`face${i}`);
            if (group != undefined)
              group.style.display = (i == face ? "inline" : "none");
          }
    }

    async cycleTo(face) {
        return new Promise(resolve => this.#cycleLoop(face, 0, resolve));
    }

    #cycleLoop(face, counter, resolve) {
        if (counter >= 6) {
            this.setVisibleFace(face);
            this.wiggle();
            setTimeout(() => { resolve(); }, 1000);
        } else {
            this.setVisibleFace(randInt(1, 6));
            setTimeout(() => { this.#cycleLoop(face, counter+1, resolve); }, 120);
        }
    }

    wiggle() {
        this.wiggleTl.clear();
        this.wiggleTl.to(this.diceSvg, {rotate: "-18deg", yoyo: true, repeat: 1, duration: 0.05})
            .to(this.diceSvg, {rotate: "20deg", yoyo: true, repeat: 1, duration: 0.05})
            .to(this.diceSvg, {rotate: "-6deg", yoyo: true, repeat: 1, duration: 0.04})
            .to(this.diceSvg, {rotate: "4deg", yoyo: true, repeat: 1, duration: 0.03})
            .to(this.diceSvg, {rotate: "0deg", duration: 0.015});
    }

    async appear() {
        this.playerNameLabel.style.visibility = "visible";
        return new Promise(resolve => {
            this.appearTl.clear();
            // this.appearTl.fromTo(this.scale, { x: 0, y: 0, z: 0, duration: 0 }, { x: 1, y: 1, z: 1, ease: Power1.easeOut, duration: 0.05, onComplete: resolve });
            this.appearTl.fromTo(this.diceSvg, { width: "0em", duration: 0 }, { width: TARGET_APPEAR_WIDTH, ease: Power1.easeOut, duration: 0.05, onComplete: resolve });
            this.wiggle();
        });
    }

    async disappear() {
        this.playerNameLabel.style.visibility = "hidden";
        return new Promise(resolve => {
            this.appearTl.clear();
            // this.appearTl.fromTo(this.scale, { x: 1, y: 1, z: 1, duration: 0 }, { x: 0, y: 0, z: 0, ease: Power1.easeOut, duration: 0.05, onComplete: resolve });
            this.appearTl.fromTo(this.diceSvg, { width: TARGET_APPEAR_WIDTH, duration: 0 }, { width: "0em", ease: Power1.easeOut, duration: 0.075, onComplete: resolve });
            this.wiggle();
        });
    }

}
