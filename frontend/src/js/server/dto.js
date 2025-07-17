import { strjoin } from "../utils";

export class MessageResponse {
    constructor(data) {
        this.message = data.message;
    }
}

export class OpponentState {
    constructor(data) {
        this.player_id = data.player_id;
        this.health = data.health;
        this.card_count = data.card_count;
        this.discard_cards = data.discard_cards;
        this.buffs = data.buffs.map(buff_data => { return new BuffInfoDTO(buff_data); });
    }
}

export class GameStatusResponse {
    constructor(data) {
        this.current_player_turn = data.current_player_turn;
        this.current_player_turn_end = data.current_player_turn_end;
        this.health = data.health;
        this.cards = data.cards;
        this.discard_cards = data.discard_cards;
        this.buffs = data.buffs.map(buff_data => { return new BuffInfoDTO(buff_data); });
        this.opponents = data.opponents.map(opp_data => { return new OpponentState(opp_data); });
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

export class PlayerBuffStatusResponse {
    constructor(data) {
        this.player_id = data.player_id;
        this.buffs = data.buffs.map(buff_data => { return new BuffInfoDTO(buff_data); });
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


export function evalOpSymbol(op) {
    switch (op) {
        case "Add": return "+";
        case "Sub": return "-";
        case "Mul": return "x";
        case "PowA": return "^";
        case "PowB": return "^";
        default: return "";
    }
}

export function buffLifeTimeDesc(lifetime) {
    switch (lifetime) {
        case "UntilNextTurnEnd": return "on the next turn";
        case "UntilUsed": return "until used";
        default: return "";
    }
}

export function formatCardElement(element) {
    switch(element.toLowerCase()) {
        case "fire": return `<span style="color: #E32620;">${element}</span>`;
        case "water": return `<span style="color: #2E4D9D;">${element}</span>`;
        case "air": return `<span style="color: #968480;">${element}</span>`;
        case "earth": return `<span style="color: #EF862A;">${element}</span>`;
        default: return element;
    }
}

export function matchingCardsDesc(elements, kinds, stars) {
    let stars_with_text = [];

    stars.forEach(star => {
        if (star == "One")
            stars_with_text.push(`${star} Star`);
        else
            stars_with_text.push(`${star} Stars`);
    });

    return strjoin([strjoin(elements.map(e => { return formatCardElement(e); }), ", "), strjoin(kinds, ", "), strjoin(stars_with_text, ", ")], ", ", true);
}


export class BuffInfoDTO {
    static ATTACK_BUFF = "AttackBuff";
    static ATTACK_BUFF_DESC_TEMPLATE(value, op, elements, kinds, stars, lifetime) {
        const desc = matchingCardsDesc(elements, kinds, stars);
        return `<b>${evalOpSymbol(op)}${value}</b> attack for ${(desc ? desc + " " : "")}cards ${buffLifeTimeDesc(lifetime)}`;
    }
    static TARGET_ALL_BUFF = "TargetAllBuff";
    static TARGET_ALL_BUFF_DESC_TEMPLATE() {
        return "Cards played on the next turn will target every players";
    }
    static PLAY_ALL_CARDS_BUFF = "PlayAllCardsBuff";
    static PLAY_ALL_CARDS_BUFF_DESC_TEMPLATE(elements, kinds, stars) {
        const desc = matchingCardsDesc(elements, kinds, stars);
        return `Play all of your ${(desc ? desc + " " : "")}cards at once on the next turn`;
    }

    constructor(data) {
        this.buff_type = data.type;
        switch (data.type) {
            case BuffInfoDTO.ATTACK_BUFF:
                this.value = data.value;
                this.op = data.op;
                this.elements = data.elements;
                this.kinds = data.kinds;
                this.stars = data.stars;
                this.lifetime = data.lifetime;
                this.description = BuffInfoDTO.ATTACK_BUFF_DESC_TEMPLATE(this.value, this.op, this.elements, this.kinds, this.stars, this.lifetime);
                break;
            case BuffInfoDTO.TARGET_ALL_BUFF:
                this.description = BuffInfoDTO.TARGET_ALL_BUFF_DESC_TEMPLATE();
                break;
            case BuffInfoDTO.PLAY_ALL_CARDS_BUFF:
                this.elements = data.elements;
                this.kinds = data.kinds;
                this.stars = data.stars;
                this.description = BuffInfoDTO.PLAY_ALL_CARDS_BUFF_DESC_TEMPLATE(this.elements, this.kinds, this.stars);
        }
    }
}
