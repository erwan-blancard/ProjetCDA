import { BoxGeometry, Mesh, MeshBasicMaterial, Object3D } from "three";
import { Card, CardPile, OpponentCard } from "./cards";


// common attributes and methods for Player and Opponent
// extends Mesh to give the PlayerObject a shape and use Raycaster to check if under mouse
export class PlayerObject extends Mesh {
    /** @type {string | null} */
    name;
    /** @type {number} */
    _health = 100;
    /** @type {Array<Card>} */
    cards = [];
    /** @type {Array<Card>} */
    discard_cards = [];
    /** @type {Card | null} */
    selected_card;
    /** @type {THREE.Scene} */
    scene;

    constructor(scene) {
        const box = new BoxGeometry(6, 0.1, 3);
        const mat = new MeshBasicMaterial( { color: 0xffffff, opacity: 0.2, transparent: true } );
        super(box, mat);

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

    updateHandCardPositions(instant=false) {
        for (let i = 0; i < this.cards.length; i++) {
            const { x, y, z } = this.getHandCardPositionByIndex(i);
            this.cards[i].goto(x, y, z, (instant ? 0.0 : 0.4));
        }
    }

    getHandCardPositionByIndex(i) {
        const space = 1.05;
        const count = this.cards.length;
        // TODO may be used to offset deck pos and player pos
        const cx = this.position.x;
        const cy = this.position.y;
        const cz = this.position.z;

        const x = cx + (i*space) - ((count-1)*space) / 2;

        return { x: x, y: cy, z: cz };
    }
    
    getDiscardCardPositionByIndex(i) {
        const x = this.position.x - 4;
        const y = 0.2 + (i * 0.015);
        const z = this.position.z - 1.5;
        return { x: x, y: y, z: z };
    }

    updateDiscardCardPositions(instant=false) {
        for (let i = 0; i < this.discard_cards.length; i++) {
            const { x, y, z } = this.getDiscardCardPositionByIndex(i);
            this.discard_cards[i].goto(x, y, z, (instant ? 0.0 : 0.4));
        }
    }

    /** @param {Array<number>} card_ids  */
    updateHandCards(card_ids) {
        this.clearSelection();

        const new_cards_arr = [];

        for (let i = 0; i < card_ids.length; i++) {
            const id = card_ids[i];
            const found = this.cards.findIndex((card, _i, _o) => {
                return card.card_id == id;
            });

            if (found != -1) {
                const found_card = this.cards.splice(found, 1)[0];
                new_cards_arr.push(found_card);
            } else {
                const card = new Card(id);
                this.scene.add(card);
                new_cards_arr.push(card);
            }
        }
        
        // remove cards
        for (let i = this.cards.length - 1; i >= 0; i--) {
            this.cards.splice(i, 1)[0].removeFromParent();
        }
        
        // push cards in our list
        new_cards_arr.forEach(card => {
            this.cards.push(card);
        });
        
        this.updateHandCardPositions();
        this.emitCardCountChange();
    }

    /** @param {Array<number>} card_ids  */
    updateDiscardCards(card_ids) {
        const new_cards_arr = [];

        for (let i = 0; i < card_ids.length; i++) {
            const id = card_ids[i];
            const found = this.discard_cards.findIndex((card, _i, _o) => {
                return card.card_id == id;
            });

            if (found != -1) {
                const found_card = this.discard_cards.splice(found, 1)[0];
                new_cards_arr.push(found_card);
            } else {
                const card = new Card(id);
                this.scene.add(card);
                new_cards_arr.push(card);
            }
        }
        
        // remove cards
        for (let i = this.discard_cards.length - 1; i >= 0; i--) {
            this.cards.splice(i, 1)[0].removeFromParent();
        }
        
        // push cards in our list
        new_cards_arr.forEach(card => {
            this.discard_cards.push(card);
        });

        this.updateDiscardCardPositions();
        this.emitDiscardCountChange();
    }

    isCardInHand(card_id, hand_index) {
        return hand_index >= 0 && hand_index < this.cards.length && this.cards[hand_index].card_id == card_id;
    }

    toggleCardSelection(index) {
        const card = this.cards[index];
        if (this.selected_card == card)
            this.selected_card = null;
        else
            this.selected_card = card;

        this.updateCardSelection();
    }

    clearSelection() {
        this.selected_card = null;
        this.updateCardSelection();
    }
    
    updateCardSelection() {
        for (let i = 0; i < this.cards.length; i++) {
            const card = this.cards[i];
            let { x, y, z } = this.getHandCardPositionByIndex(i);

            if (card == this.selected_card)
                z -= 0.5;

            card.goto(x, y, z, 0.15);
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

    /** @param {Card} card  */
    addCard(card) {
        if (card.card_id != -1) {
            this.cards.push(card);
            this.emitCardCountChange();
            this.updateHandCardPositions();
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
        this.scene.add(card);
        return card;
    }

    isCardInHand(_card_id, hand_index) {
        // card_id is ignored for opponents
        return hand_index >= 0 && hand_index < this.cards.length;
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

        this.updateHandCardPositions();
        this.emitCardCountChange();
    }

    getHandCardPositionByIndex(i) {
        const space = 1.05;
        const count = this.cards.length;
        // TODO may be used to offset deck pos and player pos
        const cx = this.position.x;
        const cy = this.position.y;
        const cz = this.position.z;

        const x = cx + ((count-1-i)*space) - ((count-1)*space) / 2;

        return { x: x, y: cy, z: cz };
    }
    
    getDiscardCardPositionByIndex(i) {
        const x = this.position.x - 4;
        const y = 0.2 + (i * 0.015);
        const z = this.position.z + 1.5;    // inverted from player
        return { x: x, y: y, z: z };
    }

}
