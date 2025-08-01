import { PlayerObject, Player, Opponent } from "../game/player";
import { CSS2DObject } from 'three-stdlib'
import { BuffInfoDTO } from "../server/dto";
import { BoxGeometry, Mesh, MeshBasicMaterial } from "three";


export const UI_DIV_CLS = "player-ui";

export const HEALTH_DIV_CLS = "player-health-ui";
export const HEALTH_ICON_CLS = "player-health-icon";
export const HEALTH_LABEL_CLS = "player-health-label";

export const CARDS_DIV_CLS = "player-cards-ui";
export const CARDS_ICON_CLS = "player-cards-icon";
export const CARDS_LABEL_CLS = "player-cards-label";

export const DISCARD_DIV_CLS = "player-discard-ui";
export const DISCARD_ICON_CLS = "player-discard-icon";
export const DISCARD_LABEL_CLS = "player-discard-label";


// Player UI that can be rendered when added to a CSS2DRenderer
export class PlayerUI extends CSS2DObject {
    /** @type PlayerObject */
    player;

    /** @type HTMLElement */
    healthLabel;
    /** @type HTMLElement */
    cardsLabel;
    /** @type HTMLElement */
    discardLabel;

    /** @type {Array<CSS2DObject>} */
    buffObjs = [];

    constructor(player) {
        const uiDiv = document.createElement("div");
        uiDiv.className = UI_DIV_CLS;
        // uiDiv.style.backgroundColor = 'transparent';

        super(uiDiv);
        this.player = player;
        this.player.add(this);

        this.createUIElements(uiDiv);

        this.player.addEventListener("healthchange", () => {
            this.onHealthChanged();
        });

        this.player.addEventListener("cardcountchange", () => {
            this.onCardCountChanged();
        });

        this.player.addEventListener("discardcountchange", () => {
            this.onDiscardCountChanged();
        });

        this.player.addEventListener("buffschange", () => {
            this.onBuffsChanged();
        })
    }

    createUIElements(uiDiv) {
        // health
        const healthDiv = document.createElement("div");
        healthDiv.className = HEALTH_DIV_CLS;

        const healthLabel = document.createElement("p");
        healthLabel.className = HEALTH_LABEL_CLS;
        healthLabel.textContent = "100";

        const healthIcon = document.createElement("p");
        healthIcon.className = HEALTH_ICON_CLS;

        healthDiv.appendChild(healthIcon);
        healthDiv.appendChild(healthLabel);
        
        // cards in hand

        const cardsDiv = document.createElement("div");
        cardsDiv.className = CARDS_DIV_CLS;

        const cardsLabel = document.createElement("p");
        cardsLabel.className = CARDS_LABEL_CLS;
        cardsLabel.textContent = "5";

        const cardsIcon = document.createElement("p");
        cardsIcon.className = CARDS_ICON_CLS;
        
        cardsDiv.appendChild(cardsIcon);
        cardsDiv.appendChild(cardsLabel);

        // cards in discard

        const discardDiv = document.createElement("div");
        discardDiv.className = DISCARD_DIV_CLS;

        const discardLabel = document.createElement("p");
        discardLabel.className = DISCARD_LABEL_CLS;
        discardLabel.textContent = "0";

        const discardIcon = document.createElement("p");
        discardIcon.className = DISCARD_ICON_CLS;

        discardDiv.appendChild(discardIcon);
        discardDiv.appendChild(discardLabel);

        this.healthLabel = healthLabel;
        this.cardsLabel = cardsLabel;
        this.discardLabel = discardLabel;

        uiDiv.appendChild(healthDiv);
        uiDiv.appendChild(cardsDiv);
        uiDiv.appendChild(discardDiv);
    }

    onHealthChanged() {
        // TODO animate
        this.healthLabel.textContent = this.player.health;
    }

    onCardCountChanged() {
        // TODO animate
        this.cardsLabel.textContent = this.player.cards.length;
    }

    onDiscardCountChanged() {
        // TODO animate
        this.discardLabel.textContent = this.player.discard_cards.length;
    }

    onBuffsChanged() {
        const buffs = this.player.buffs;

        // clear previous buff info objs
        this.buffObjs.forEach(obj => {
            obj.removeFromParent();
        });
        this.buffObjs.length = 0;

        for (let i = 0; i < buffs.length; i++) {
            const buffObj = new BuffInfo(buffs[i]);
            buffObj.position.x -= 1;
            // compensate for POV offset if Player FIXME
            const offsetZ = (this.parent instanceof Player ? 0.55 : 0.70);
            buffObj.position.z -= (offsetZ * i);
            this.add(buffObj);
            this.buffObjs.push(buffObj);
        }
        
    }

}


export class BuffInfo extends CSS2DObject {
    /** @type {BuffInfoDTO} */
    info;
    hoverObj;

    constructor(info) {
        const element = document.createElement("p");
        super(element);
        // used to target buff info objects with raycaster -> TODO enum of layers ?
        this.layers.set(1);
        this.update(info);

        this.#createHoverObj();
        
    }

    #createHoverObj() {
        const box = new BoxGeometry(0.5, 0.0, 0.5);
        const mat = new MeshBasicMaterial( { color: 0xffffff, opacity: 0.0, transparent: true } );
        this.hoverObj = new Mesh(box, mat);
        this.hoverObj.layers.set(1);
        this.hoverObj.position.z += 0.5;    // center on element FIXME improve
        this.add(this.hoverObj);
    }

    update(info) {
        this.info = info;
        this.element.classList.add("buff", this.info.buff_type);
    }
}
