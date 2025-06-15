import { Object3D } from "three";
import { Card, OpponentCard } from "./cards";


// common attributes and methods for Player and Opponent
export class PlayerObject extends Object3D {
    /** @type {string | null} */
    name;
    /** @type number */
    _health = 100;
    /** @type Array<Card> */
    cards = [];
    /** @type Array<Card> */
    discard_cards;

    /** @type THREE.Scene */
    scene;

    constructor(scene) {
        super();

        this.scene = scene;
        this.scene.add(this);
    }

    get health() {
        return this._health;
    }

    set health(value) {
        if (value < 0) { value = 0; }
        this._health = value;
        this.emitHealthChange();
    }

    updateCardPositions(instant=false) {
        const space = 1.05;
        const count = this.cards.length;
        // TODO may be used to offset deck pos and player pos
        const cx = this.position.x;
        const cy = this.position.y;
        const cz = this.position.z;

        for (let i = 0; i < this.cards.length; i++) {
            const x = cx + (i*space) - ((count-1)*space) / 2;
            this.cards[i].goto(x, cy, cz, (instant ? 0.0 : 0.4));
        }
    }

    emitHealthChange() {
        this.dispatchEvent({type: "healthchange"});
    }

    emitCardCountChange() {
        this.dispatchEvent({type: "cardcountchange"})
    }

    emitDiscardCountChange() {
        this.dispatchEvent({type: "discardcountchange"})
    }
}


export class Player extends PlayerObject {

}


export class Opponent extends PlayerObject {
    /** @type Array<OpponentCard> */
    cards = [];

    constructor(scene, card_count=5) {
        super(scene);
        
        // fill deck with 5 cards
        for (let i = 0; i < card_count; i++) {
            this.cards.push(this.#createOpponentCard());
        }
        this.emitCardCountChange();
    }

    // create an OpponentCard object and add it to the scene
    #createOpponentCard() {
        const card = new OpponentCard();
        this.add(card);
        return card;
    }

    setCardCount(count) {
        const prev_count = this.cards.length;
        
        if (prev_count < count) {
            this.cards.length = count;  // resize the array

            for (let i = prev_count; i < count; i++) {
                this.cards[i] = this.#createOpponentCard();
            }
        } else if (prev_count > count) {
            // remove cards as they are still referenced by the scene
            for (let i = prev_count; i > count; i--) {
                this.cards.pop().removeFromParent();
            }
            this.cards.length = count;  // resize the array
        }

        this.updateCardPositions();
        this.emitCardCountChange();
    }

}
