import { Card, OpponentCard } from "../cards";
import { Opponent, Player, PlayerObject } from "../player";
import { EventMgr } from "./event_mgr";
import * as THREE from 'three';
import * as GAME from "../game";
import { GameStatusResponse } from "../../server/dto";
import gsap from "gsap";
import { sleep } from "../../utils";


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
    async run() {}

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

    async run() {
        // TODO animate
        this.player.health -= this.amount;
    }

}


export class HealPlayerEvent extends PlayerEvent {

    constructor(player, amount) {
        super(player);
        this.amount = amount;
    }

    async run() {
        // TODO animate
        this.player.health += this.amount;
    }

}


export class ThrowDiceEvent extends PlayerEvent {

    constructor(player, result) {
        super(player);
        this.result = result;
        this.timeout = -1;
    }

    async run() {
        try {
            GAME.dice.setPlayerName(this.player.name);
        } catch (e) {
            console.log("Exception when setting dice player name:", e);
            GAME.dice.setPlayerName("N/A");
        }
        GAME.dice.appear();
        await GAME.dice.cycleTo(this.result);
        await GAME.dice.disappear();
        this.onTimeout();
    }

}


export class DrawCardEvent extends PlayerEvent {

    constructor(player, card_id) {
        super(player);
        this.card_id = card_id;
    }

    async run() {
        if (this.player == GAME.PLAYER) {
            const card = new Card(this.card_id);
            this.player.addCard(card);
            // add card to scene
            GAME.scene.add(card);
            GAME.cardPile.count -= 1;
        } else if (this.player != null) {
            // opponent, card_id is -1
            this.player.setCardCount(this.player.cards.length + 1);
        }
    }

}

export class DiscardCardEvent extends PlayerEvent {

    constructor(player, card_index) {
        super(player);
        this.card_index = card_index;
    }

    async run() {
        if (this.player != null) {
            this.player.cards.splice(this.card_index, 1).forEach((card) => {
                this.player.discard_cards.push(card);
            });
            
            this.player.updateHandCardPositions();
            this.player.emitCardCountChange();
            this.player.updateDiscardCardPositions();
            this.player.emitDiscardCountChange();
        }
    }

}


// event to move card towards the center of the playing field
export class PutCardForward extends CardEvent {

    constructor(card, display_card_id=-1) {
        super(card);
        this.display_card_id = display_card_id;
    }

    async run() {
        if (this.card != null) {
            this.card.active = true; // prevent position updates with updateHandCardPositions() and updateDiscardCardPositions()

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
    
    constructor(player, card) {
        super();
        this.timeout = 400;
        this.player = player;
        this.card = card;
    }

    async run() {
        if (this.card != null) {
            this.card.active = false;
            // transfer card to discard pile
            this.player.discard_cards.push(this.card);
            // remove card from hand if it's not fake
            const idx = this.player.cards.indexOf(this.card);
            if (idx != -1)
                this.player.cards.splice(idx, 1);
            this.player.updateHandCardPositions();
            this.player.emitCardCountChange();
            this.player.updateDiscardCardPositions();
            this.player.emitDiscardCountChange();
        }
    }
}


export class ChangeTurnEvent extends PlayerEvent {

    constructor(player, turn_end=0) {
        super(player);
        this.turn_end = turn_end;
        this.timeout = 150;
    }

    async run() { GAME.updateCurrentPlayerTurn(this.player, this.turn_end); }

}


export class PlayerBuffsUpdateEvent extends PlayerEvent {

    constructor(player, buffs) {
        super(player);
        this.buffs = buffs;
        this.timeout = 0;
    }

    async run() {
        GAME.buffTooltip.visible = false;
        if (this.player)
            this.player.updateBuffs(this.buffs);
    }
}


export class CollectDiscardCardsEvent extends GameEvent {
    constructor(cards_in_pile) {
        super();
        this.timeout = -1;
        this.cards_in_pile = cards_in_pile;
    }
    
    async run() {
        let cards_remain = true;
        while (cards_remain) {
            if (GAME.PLAYER.discard_cards.length > 0) {
                const tl = gsap.timeline();
                const card = GAME.PLAYER.discard_cards.pop();
                tl.to(card.position, { x: 0.0, y: 0.0, z: 0.0, duration: 0.5, onComplete: () => {
                    card.removeFromParent();
                    GAME.cardPile.count += 1;
                } });
            } else {
                cards_remain = false;
            }

            let cards_remain_for_opponents = 0;
            for (const opponent of GAME.OPPONENTS.values()) {
                if (opponent.discard_cards.length > 0) {
                    cards_remain_for_opponents += 1;
                    const tl = gsap.timeline();
                    const card = opponent.discard_cards.pop();
                    tl.to(card.position, { x: 0.0, y: 0.0, z: 0.0, duration: 0.5, onComplete: () => {
                        card.removeFromParent();
                        GAME.cardPile.count += 1;
                    } });
                }
            }

            cards_remain = GAME.PLAYER.discard_cards.length > 0 || cards_remain_for_opponents > 0;
            if (cards_remain)
                await sleep(100);
            else
                await sleep(550);   // wait 0.5 secs to complete animations
        }

        this.onTimeout();
    }

    onTimeout() {
        GAME.cardPile.count = this.cards_in_pile;
        this.mgr.executeNext();
    }

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

    async run() {
        const data = this.upd_data;

        GAME.buffTooltip.visible = false;

        try {
            GAME.PLAYER.health = data.health;
            GAME.PLAYER.updateHandCards(data.cards);
            GAME.PLAYER.updateDiscardCards(data.discard_cards);
            GAME.PLAYER.updateBuffs(data.buffs);

            // update opponents
            data.opponents.forEach(opponent_data => {
                const opponent = GAME.OPPONENTS.get(opponent_data.player_id);
                if (opponent != null) {
                    opponent.health = opponent_data.health;
                    opponent.setCardCount(opponent_data.card_count);
                    opponent.updateDiscardCards(opponent_data.discard_cards);
                    opponent.updateBuffs(opponent_data.buffs);
                }
            });

            GAME.cardPile.count = data.cards_in_pile;

            GAME.updateCurrentPlayerTurn(GAME.getPlayerById(data.current_player_turn), data.current_player_turn_end);
        } catch (e) {
            console.log("Exception when handling game update data:", e);
        }
    }

}

export class GameEndEvent extends GameEvent {
    constructor(winner_id) {
        super();
        this.timeout = -1;
        this.winner_id = winner_id;
    }

    async run() {
        GAME.displayGameEndScreen(this.winner_id);
    }

}
