import { CSS2DObject } from "three-stdlib";


export const CONTAINER_CLS = "card-tooltip";
export const NAME_CLS = "card-tooltip-name";
export const DESC_CLS = "card-tooltip-desc";


export class CardTooltip extends CSS2DObject {
    card = null;
    containerElement;
    nameElement;
    descElement;

    constructor(scene) {
        const containerElement = document.createElement("div");
        containerElement.className = CONTAINER_CLS;
        super(containerElement);

        scene.add(this);

        this.containerElement = containerElement;
        this.nameElement = document.createElement("p");
        this.nameElement.className = NAME_CLS;
        this.descElement = document.createElement("p");
        this.descElement.className = DESC_CLS;

        containerElement.appendChild(this.nameElement);
        containerElement.appendChild(this.descElement);

        this.update();
    }

    update(card=null) {
        if (card != null)
            this.card = card;

        if (this.card != null && this.card.info != null) {
            this.nameElement.textContent = this.card.info.name;
            this.descElement.textContent = this.card.info.desc;
            this.containerElement.style.backgroundColor = this.card.info.color;
        } else {
            this.nameElement.textContent = "Invalid";
            this.descElement.textContent = "Invalid";
            this.containerElement.style.backgroundColor = "red";
        }
    }
}