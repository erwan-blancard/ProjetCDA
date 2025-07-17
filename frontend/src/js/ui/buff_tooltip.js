import { CSS2DObject } from "three-stdlib";
import { BuffInfo } from "./player_ui";


export const CONTAINER_CLS = "buff-tooltip";
export const DESC_CLS = "buff-tooltip-desc";


export class BuffTooltip extends CSS2DObject {
    /** @type {BuffInfo | null} */
    buff = null;
    containerElement;
    descElement;

    constructor(scene) {
        const containerElement = document.createElement("div");
        containerElement.className = CONTAINER_CLS;
        super(containerElement);

        scene.add(this);

        this.containerElement = containerElement;
        this.descElement = document.createElement("p");
        this.descElement.className = DESC_CLS;

        containerElement.appendChild(this.descElement);

        this.update();
    }

    update(buff=null) {
        if (buff != null)
            this.buff = buff;

        if (this.buff != null && this.buff.info != null) {
            this.descElement.innerHTML = this.buff.info.description;
            this.containerElement.style.backgroundColor = "lightgray";
        } else {
            this.descElement.innerHTML = "Invalid";
            this.containerElement.style.backgroundColor = "red";
        }
    }
}