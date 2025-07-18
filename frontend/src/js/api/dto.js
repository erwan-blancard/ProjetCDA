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
    id;
    users;
    users_ready;
    unlisted;
    game_id;

    constructor(data) {
        this.id = data.id;
        this.users = data.users;
        this.users_ready = data.users_ready;
        this.unlisted = data.unlisted;
        this.game_id = data.game_id;
    }

    is_user_ready(user_id) {
        return this.users_ready.indexOf(user_id) != -1;
    }
}

export class LobbyInfoDTO {
    id;
    users;
    users_ready;
    ingame;

    constructor(data) {
        this.id = data.id;
        this.users = data.users;
        this.users_ready = data.users_ready;
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
            this.entries.push(new LobbyInfoDTO(entry));
        });

        this.page = data.page;
        this.page_count = data.page_count;
    }
}
