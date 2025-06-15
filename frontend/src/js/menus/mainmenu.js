import { get_current_game_info } from '../api/account';
import { LobbyDTO, LobbyInfoDTO } from '../api/dto';
import { LobbyList } from '../ui/lobby_list';
import { ViewMgr } from '../ui/viewmgr';
import { login_guard } from '../utils';

// const account = await login_guard();

// try {
//     // check if a game is running
//     if (await get_current_game_info() != null) {
//         window.location.href = "/ingame.html";
//     }
// } catch (error) {}

const mainview = document.getElementById("main");
const createlobbyview = document.getElementById("create-lobby");
const selectlobbyview = document.getElementById("select-lobby");

const viewMgr = new ViewMgr([mainview, createlobbyview, selectlobbyview]);

// const lobbyList = new LobbyList();

document.getElementById("to-create-lobby").onclick = () => {
    viewMgr.setPrimaryView(createlobbyview);
};
document.getElementById("to-join-lobby").onclick = () => {
    viewMgr.setPrimaryView(selectlobbyview);
};
document.getElementById("create-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};
document.getElementById("select-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};

// document.replaceChild(document.getElementById("lobby-list"), lobbyList);

/** @type LobbyList */
const lobbyList = document.getElementById("lobby-list");

document.getElementById("test-add-lobby").onclick = () => {
    lobbyList.push(new LobbyInfoDTO({users: ["Player 1", "Player 2", "Player 3", "Player 4", "Player 5", "Player 6"], users_ready: [], password: true, ingame: false}));
};
document.getElementById("test-remove-lobby").onclick = () => {
    lobbyList.remove_index(lobbyList.count()-1);
}
document.getElementById("test-clear-list").onclick = () => {
    lobbyList.clear();
}

// lobbyList.push(new LobbyInfoDTO({users: ["Player 1", "Player 2", "Player 3"], users_ready: [], password: false, ingame: false}));
// lobbyList.push(new LobbyInfoDTO({users: ["Player 1", "Player 2", "Player 3", "Player 4", "Player 5", "Player 6"], users_ready: [], password: true, ingame: false}));
// lobbyList.push(new LobbyInfoDTO({users: ["Player 1", "Player 2", "Player 3", "Player 4", "Player 5", "Player 6", "Player 7"], users_ready: [], password: true, ingame: false}));
// lobbyList.push(new LobbyInfoDTO({users: ["Player 1", "Player 2", "Player 3", "Player 4", "Player 5", "Player 6", "Player 7", "Player 8", "Player 8"], users_ready: [], password: true, ingame: false}));
// lobbyList.push(new LobbyInfoDTO({users: ["Player 1", "Player 2", "Player 3", "Player 4", "Player PLAYERPLAYER", "Player 6", "Player 7", "Player 8", "Player 8"], users_ready: [], password: true, ingame: false}));
