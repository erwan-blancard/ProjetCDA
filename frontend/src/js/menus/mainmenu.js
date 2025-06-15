import { get_current_game_info } from '../api/account';
import { LobbyDTO, LobbyEntryDTO, LobbyInfoDTO } from '../api/dto';
import { LobbyList } from '../ui/lobby_list';
import { ViewMgr } from '../ui/viewmgr';
import { login_guard } from '../utils';
import { create_lobby, list_lobbies } from '../api/lobby';
import { displayYesNo } from '../ui/popup';


const account = await login_guard();

try {
    // check if a game is running
    if (await get_current_game_info() != null) {
        window.location.href = "/ingame.html";
    }
} catch (error) {}

const mainview = document.getElementById("main");
const createlobbyview = document.getElementById("create-lobby");
const selectlobbyview = document.getElementById("select-lobby");
const currentlobbyview = document.getElementById("current-lobby");

const viewMgr = new ViewMgr();

/** @type LobbyList */
const lobbyList = document.getElementById("lobby-list");

lobbyList.lobbyJoinedCallback = (lobby) => {
    console.log("Lobby Joined: ", lobby);

    viewMgr.setPrimaryView(currentlobbyview);
};


document.getElementById("to-create-lobby").onclick = () => {
    viewMgr.setPrimaryView(createlobbyview);
};
document.getElementById("to-join-lobby").onclick = () => {
    viewMgr.setPrimaryView(selectlobbyview);

    // refresh list
    lobbyList.refreshPage();
};


const lobbyPasswordCheck = document.getElementById("lobby-private-check");
const lobbyPasswordInput = document.getElementById("lobby-password-input");
lobbyPasswordInput.disabled = true;

lobbyPasswordCheck.onclick = () => {
    lobbyPasswordInput.disabled = !lobbyPasswordCheck.checked;
};


document.getElementById("create-lobby-validate-button").onclick = () => {
    const use_password = lobbyPasswordCheck.checked;
    const password = lobbyPasswordInput.value;
    const lobby = create_lobby(use_password ? password : null);

    if (lobby)
        viewMgr.setPrimaryView(currentlobbyview);
};
document.getElementById("create-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};


document.getElementById("select-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};


document.getElementById("current-lobby-ready-button").onclick = () => {
    
};
document.getElementById("current-lobby-leave-button").onclick = () => {
    displayYesNo("Leave this lobby ?", "", () => {
        viewMgr.setPrimaryView(mainview);
    });
};
