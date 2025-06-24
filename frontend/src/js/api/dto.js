export class AccountDTO {
    id;
    username;

    constructor(data) {
        this.id = data.id;
        this.username = data.username;
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

    is_user_ready(user_id) {
        return this.users_ready.indexOf(user_id) != -1;
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
    // lobby_info;
    users;
    users_ready;
    /** @type boolean */
    password;
    ingame;

    constructor(data) {
        this.lobby_id = data.lobby_id;
        // this.lobby_info = new LobbyInfoDTO(data);   // lobby info fields are flattened by api
        this.users = data.users;
        this.users_ready = data.users_ready;
        this.password = data.password;
        this.ingame = data.ingame;
    }
}

export class LobbyPageListDTO {
    entries;
    page;
    page_count;

    constructor(data) {
        this.entries = [];

        data.entries.forEach(entry => {
            this.entries.push(new LobbyEntryDTO(entry));
        });

        this.page = data.page;
        this.page_count = data.page_count;
    }
}
