export class MessageResponse {
    constructor(data) {
        this.message = data.message;
    }
}

export class GameStatusResponse {
    constructor(data) {
        this.current_player_turn = data.current_player_turn;
        this.current_player_turn_end = data.current_player_turn_end;
        this.health = data.health;
        this.cards = data.cards;
        this.discard_cards = data.discard_cards;
        this.opponents = data.opponents;
        this.cards_in_pile = data.cards_in_pile;
    }
}

export class SessionInfoResponse {
    constructor(data) {
        this.id = data.id;
        this.players = data.players;
    }
}

export class PlayCardResponse {
    /** @type {Array<PlayActionDTO>} */
    actions;
    constructor(data) {
        this.player_id = data.player_id;
        this.card_id = data.card_id;
        this.hand_index = data.hand_index;
        this.actions = data.actions.map((action_data) => { return new PlayActionDTO(action_data); });
    }
}

export class DrawCardResponse {
    constructor(data) {
        this.player_id = data.player_id;
        this.card_id = data.card_id;
    }
}

export class ChangeTurnResponse {
    constructor(data) {
        this.player_id = data.player_id;
        this.turn_end = data.turn_end;
    }
}

export class CollectDiscardCardsResponse {
    constructor(data) {
        this.cards_in_pile = data.cards_in_pile;
    }
}

export class GameEndResponse {
    constructor(data) {
        this.winner_id = data.winner_id;
    }
}


// PlayInfo DTOs

export class PlayActionDTO {
    /** @type {Array<ActionTargetDTO>} */
    targets;
    constructor(data) {
        this.dice_roll = data.dice_roll;
        this.player_dice_id = data.player_dice_id;
        this.targets = data.targets.map(target_data => { return new ActionTargetDTO(target_data); });
    }
}

export class ActionTargetDTO {
    /** @type {ActionTypeDTO} */
    action;
    constructor(data) {
        this.player_id = data.player_id;
        this.action = new ActionTypeDTO(data.action);
        this.effect = data.effect;
    }
}

export class ActionTypeDTO {
    static ATTACK = "Attack";
    static HEAL = "Heal";
    static DRAW = "Draw";
    static DISCARD = "Discard";

    constructor(data) {
        this.type = data.type;
        switch (data.type) {
            case ActionTypeDTO.ATTACK:
                this.amount = data.amount;
                break;
            case ActionTypeDTO.HEAL:
                this.amount = data.amount;
                break;
            case ActionTypeDTO.DRAW:
                this.cards = data.cards;
                break;
            case ActionTypeDTO.DISCARD:
                this.cards = data.cards;
                break;
            default:
                break;
        }
    }
}

