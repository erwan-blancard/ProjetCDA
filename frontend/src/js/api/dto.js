export class AccountDTO {
    id;
    username;
    email;

    constructor(data) {
        this.id = data.id;
        this.username = data.username;
        this.email = data.email;
    }
}

export class GameSessionInfoDTO {
    game_id;
    players;

    constructor(data) {
        this.game_id = data.game_id;
        this.players = data.players;
    }
}


/* Lobby DTO */

export class LobbyDTO {
    users;
    users_ready;
    password;
    game_id;

    constructor(data) {
        this.users = data.users;
        this.users_ready = data.users_ready;
        this.password = data.password;
        this.game_id = data.game_id;
    }
}

export class LobbyInfoDTO {
    users;
    users_ready;
    /** @type boolean */
    password;
    ingame;

    constructor(data) {
        this.users = data.users;
        this.users_ready = data.users_ready;
        this.password = data.password;
        this.ingame = data.ingame;
    }
}

export class LobbyEntryDTO {
    lobby_id;
    lobby_info;

    constructor(data) {
        this.lobby_id = data.lobby_id;
        this.lobby_info = new LobbyInfoDTO(data);   // lobby info fields are flattened by api
    }
}
