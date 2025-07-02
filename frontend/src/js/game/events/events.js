import { Card, OpponentCard } from "../cards";
import { Opponent, Player, PlayerObject } from "../player";
import { EventMgr } from "./event_mgr";
import * as THREE from 'three';
import * as GAME from "../game";
import { GameStatusResponse } from "../../server/dto";


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

    constructor(card, display_card_id=-1) {
        super(card);
        this.display_card_id = display_card_id;
    }

    run() {
        if (this.card != null) {
            if (this.card instanceof OpponentCard) {
                // change display of OpponentCard to match the expected card's look
                this.card.displayCardAsFront(this.display_card_id);
                this.card.flipCard();
            }

            const pos = this.card.position;
            let new_pos = new THREE.Vector3(pos.x, pos.y, pos.z);
            new_pos.z += (this.card instanceof OpponentCard ? 2 : -2);
            new_pos.y += 0.5;
            this.card.goto(new_pos.x, new_pos.y, new_pos.z, this.timeout / 1000.0);
        }
    }
}

export class PutCardInPile extends GameEvent {
    
    constructor(player, card, is_fake=false) {
        super();
        this.timeout = 400;
        this.player = player;
        this.card = card;
        this.is_fake = is_fake;
    }

    run() {
        if (this.card != null) {
            // transfer card to discard pile
            this.player.discard_cards.push(this.card);
            // remove card from hand if it's not fake
            if (!this.is_fake) {
                this.player.cards.splice(this.player.cards.indexOf(this.card), 1);
            }
            this.player.updateHandCardPositions();
            this.player.emitCardCountChange();
            this.player.updateDiscardCardPositions();
            this.player.emitDiscardCountChange();
        }
    }
}


export class ChangeTurnEvent extends PlayerEvent {

    constructor(player) {
        super(player);
        this.timeout = 150;
    }

    run() { GAME.updateCurrentPlayerTurn(this.player); }

}


/**
 * Event for GameStatus updates
 */
export class GameUpdateEvent extends GameEvent {
    /** @param {GameStatusResponse} upd_data  */
    upd_data;

    constructor(data) {
        super();
        this.upd_data = data;
        // this.timeout = 0;
    }

    run() {
        const data = this.upd_data;

        try {
            GAME.PLAYER.health = data.health;
            GAME.PLAYER.updateHandCards(data.cards);
            GAME.PLAYER.updateDiscardCards(data.discard_cards);

            // update opponents
            data.opponents.forEach(opponent_data => {
                const opponent = GAME.OPPONENTS.get(opponent_data.player_id);
                if (opponent != null) {
                    opponent.health = opponent_data.health;
                    opponent.setCardCount(opponent_data.card_count);
                    opponent.updateDiscardCards(opponent_data.discard_cards);
                }
            });

            GAME.cardPile.count = data.cards_in_pile;

            GAME.updateCurrentPlayerTurn(GAME.getPlayerById(data.current_player_turn), data.current_player_turn_end);
        } catch (e) {
            console.log("Exception when handling game update data:", e);
        }
    }

}

