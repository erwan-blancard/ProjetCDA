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
    discard_cards = [];

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

    updateDiscardCardPositions(instant=false) {
        // TODO
    }

    /** @param {Array<number>} card_ids  */
    updateHandCards(card_ids) {
        const new_card_ids = [];
        const not_seen_cards = [];

        this.cards.forEach(card => {
            not_seen_cards.push(card);
        });

        for (let i = 0; i < card_ids.length; i++) {
            const id = card_ids[i];
            const found = not_seen_cards.findIndex((card, _i, _o) => {
                return card.card_id == id;
            });

            if (found != -1) {
                not_seen_cards.splice(found, 1);
            } else {
                new_card_ids.push(id);
            }
        }
        
        // remove cards
        not_seen_cards.forEach(card => {
            this.cards.splice(this.cards.indexOf(card), 1)[0].removeFromParent();
        });

        new_card_ids.forEach(id => {
            const card = new Card(id);
            this.scene.add(card);
            this.cards.push(card);
        });
        

        this.updateCardPositions();
        this.emitCardCountChange();
    }

    /** @param {Array<number>} card_ids  */
    updateDiscardCards(card_ids) {
        const new_card_ids = [];
        const not_seen_cards = [];

        this.discard_cards.forEach(card => {
            not_seen_cards.push(card);
        });

        for (let i = 0; i < card_ids.length; i++) {
            const id = card_ids[i];
            const found = not_seen_cards.findIndex((card, _i, _o) => {
                return card.card_id == id;
            });

            if (found != -1) {
                not_seen_cards.splice(found, 1);
            } else {
                new_card_ids.push(id);
            }
        }
        
        // remove cards
        not_seen_cards.forEach(card => {
            this.discard_cards.splice(this.discard_cards.indexOf(card), 1)[0].removeFromParent();
        });

        new_card_ids.forEach(id => {
            const card = new Card(id);
            this.scene.add(card);
            this.discard_cards.push(card);
        });

        this.updateDiscardCardPositions();
        this.emitDiscardCountChange();
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

    /** @param {Card} card  */
    addCard(card) {
        if (card.card_id != -1) {
            this.cards.push(card);
            this.emitCardCountChange();
            this.updateCardPositions();
        }
    }

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
