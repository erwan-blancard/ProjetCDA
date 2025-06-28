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
    constructor(data) {
        this.player_id = data.player_id;
        this.card_id = data.card_id;
        this.hand_index = data.hand_index;
        this.actions = data.actions;
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
    }
}
