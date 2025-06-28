import { Card, OpponentCard } from "../cards";
import { Opponent, Player, PlayerObject } from "../player";
import { EventMgr } from "./event_mgr";
import * as THREE from 'three';
import * as GAME from "../game";


export class GameEvent {
    /**
     * @type {EventMgr | null}
     * set by EventMgr when pushed to the queue
     */
    mgr = null;
    timeout = 250;    // in ms
    #started = 0;

    constructor() {}

    // reimplement logic here
    run() {}

    // read-only
    get started() { return this.#started; }

    /**
     * called by EventMgr
     * do not reimplement this function, reimplement run() instead
     */
    execute() {
        // if timeout < 0, it's up to run() to call notifyMgr() when done
        if (this.timeout >= 0)
            setTimeout(() => { this.onTimeout(); }, this.timeout);
        this.#started = Date.now();
        this.run();
    }

    // tells the EventMgr to execute next event in queue
    onTimeout() { this.mgr.executeNext(); }

}


export class PlayerEvent extends GameEvent {
    /** @type {PlayerObject | Player | Opponent} */
    player;

    constructor(player) {
        super();
        this.player = player;
    }
}

export class CardEvent extends GameEvent {
    /** @type {Card} */
    card;

    constructor(card) {
        super();
        this.card = card;
    }
}


export class DamagePlayerEvent extends PlayerEvent {

    constructor(player, amount) {
        super(player);
        this.amount = amount;
    }

    run() {
        // TODO animate
        this.player.health -= this.amount;
    }

}


export class HealPlayerEvent extends PlayerEvent {

    constructor(player, amount) {
        super(player);
        this.amount = amount;
    }

    run() {
        // TODO animate
        this.player.health += this.amount;
    }

}


export class ThrowDiceEvent extends PlayerEvent {

    constructor(player, result) {
        super(player);
        this.result = result;
    }

    run() {
        // TODO animate
    }

}


export class DrawCardEvent extends PlayerEvent {

    constructor(player, card_id) {
        super(player);
        this.card_id = card_id;
    }

    run() {
        if (this.player == GAME.PLAYER) {
            const card = new Card(this.card_id);
            this.player.addCard(card);
            // add card to scene
            GAME.scene.add(card);
        } else if (this.player != null) {
            // opponent, card_id is -1
            this.player.setCardCount(this.player.cards.length + 1);
        }
    }

}


// event to move card towards the center of the playing field
export class PutCardForward extends CardEvent {

    run() {
        if (this.card != null) {
            if (this.card instanceof OpponentCard)
                this.card.flipCard();

            const pos = this.card.position;
            let new_pos = new THREE.Vector3(pos.x, pos.y, pos.z);
            new_pos.z += (this.card instanceof OpponentCard ? 2 : -2);
            new_pos.y += 0.5;
            this.card.goto(new_pos.x, new_pos.y, new_pos.z, this.timeout / 1000.0);
        }
    }
}

export class PutCardDown extends CardEvent {
    
    constructor(card, is_fake=false) {
        super(card);
        this.is_fake = is_fake;
    }

    run() {
        if (this.card != null) {
            if (this.card instanceof OpponentCard)
                this.card.flipCard();

            const pos = this.card.position;
            let new_pos = new THREE.Vector3(pos.x, pos.y, pos.z);
            new_pos.z -= (this.card instanceof OpponentCard ? 2 : -2);
            new_pos.y -= 0.5;
            this.card.goto(new_pos.x, new_pos.y, new_pos.z, this.timeout / 1000.0);
        }
    }

    onTimeout() {
        // reset cover for opponent cards
        if (this.card instanceof OpponentCard)
            this.card.displayCoverAsFront();
        if (this.is_fake)
            this.card.removeFromParent();
        super.onTimeout();
    }

}

