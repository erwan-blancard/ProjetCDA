import { CSS2DObject } from "three-stdlib";
import { formatCardElement } from "../server/dto";
import { Card } from "../game/cards";


export const CONTAINER_CLS = "card-tooltip";
export const NAME_CLS = "card-tooltip-name";
export const DESC_CLS = "card-tooltip-desc";

export const ATTR_CONTAINER_CLS = "card-tooltip-attr-container";


export class CardTooltip extends CSS2DObject {
    /** @type {Card | null} */
    card = null;
    containerElement;
    nameElement;

    attributesContainer;
    kindLabel;
    starsLabel;
    elementLabel;

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

        this.attributesContainer = document.createElement("div");
        this.attributesContainer.className = ATTR_CONTAINER_CLS;
        this.kindLabel = document.createElement("p");
        this.starsLabel = document.createElement("p");
        this.elementLabel = document.createElement("p");

        this.attributesContainer.appendChild(this.kindLabel);
        this.attributesContainer.appendChild(this.starsLabel);
        this.attributesContainer.appendChild(this.elementLabel);

        containerElement.appendChild(this.nameElement);
        containerElement.appendChild(this.attributesContainer);
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
            
            this.elementLabel.innerHTML = formatCardElement(this.card.info.element);
            this.kindLabel.textContent = this.card.info.kind;
            // compensate for star emoji space
            this.starsLabel.innerHTML = this.card.info.stars + ` <span style="font-size: 0.8em; text-align: center">⭐</span>`;
        } else {
            this.nameElement.textContent = "Invalid";
            this.descElement.textContent = "Invalid";
            this.containerElement.style.backgroundColor = "red";

            this.elementLabel.innerHTML = "";
            this.kindLabel.textContent = "";
            this.starsLabel.textContent = "";
        }
    }
}