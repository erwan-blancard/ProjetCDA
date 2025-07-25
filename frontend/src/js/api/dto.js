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


export class FriendDTO {
    id;
    account1;
    account2;
    status;

    constructor(data) {
        this.id = data.id;
        this.account1 = data.account1;
        this.account2 = data.account2;
        this.status = data.status;
    }
}


export class FriendWithLobbyStatusDTO {
    id;
    account_id;
    username;
    lobby_id;

    constructor(data) {
        this.id = data.id;
        this.account_id = data.account_id;
        this.username = data.username;
        this.lobby_id = data.lobby_id;
    }
}


export class AccountStatsDTO {
    id;
    account_id;
    first_log;
    last_log;
    games_played;
    games_won;
    wallet;
    experience;
    level;
    season_rank;
    best_rank;

    constructor(data) {
        this.id = data.id;
        this.account_id = data.account_id;
        this.first_log = data.first_log;
        this.last_log = data.last_log;
        this.games_played = data.games_played;
        this.games_won = data.games_won;
        this.wallet = data.wallet;
        this.experience = data.experience;
        this.level = data.level;
        this.season_rank = data.season_rank;
        this.best_rank = data.best_rank;
    }
}
