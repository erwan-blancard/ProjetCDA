import { Card } from "../cards";
import { Opponent, Player, PlayerObject } from "../player";
import { EventMgr } from "./event_mgr";
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
            setTimeout(() => { this.notifyMgr(); }, this.timeout);
        this.#started = Date.now();
        this.run();
    }

    // tells the EventMgr to execute next event in queue
    notifyMgr() { this.mgr.executeNext(); }

}


// convenient class
export class PlayerEvent extends GameEvent {
    /** @type {PlayerObject | Player | Opponent} */
    player;

    constructor(player) {
        super();
        this.player = player;
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
