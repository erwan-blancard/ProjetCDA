import { get_current_game_info } from '../api/account';
import { LobbyDTO, LobbyEntryDTO, LobbyInfoDTO } from '../api/dto';
import { LobbyList } from '../ui/lobby_list';
import { ViewMgr } from '../ui/viewmgr';
import { api_url, login_guard } from '../utils';
import { create_lobby, get_current_lobby, list_lobbies } from '../api/lobby';
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



let current_lobby = null;

try {
    // check
    current_lobby = await get_current_lobby()
    if (current_lobby != null) {
        viewMgr.setPrimaryView(currentlobbyview);
    }
} catch (error) {}


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
    const lobby = create_lobby(use_password ? password : null, true);

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


function handle_friend_request_update(request_id, user_id, status) {
    console.log(`request_id: ${request_id}, user_id: ${user_id}, status: ${status}`);
}

function handle_lobby_user_list_change(user_ids) {
    console.log(`user_ids: ${user_ids}`);
}

function handle_lobby_user_ready_change(user_id, ready) {
    console.log(`user_id: ${user_id}, ready: ${ready}`);
}


let events;

if (account != null) {
    events = new EventSource(api_url("/events"), { withCredentials: true });

    events.onopen = () => {
        console.log("Listening sse...");
    }

    events.onerror = (ev) => {
        console.log("sse error:", ev);
    };

    events.onmessage = (ev) => {
        const json_data = ev.data

        if (typeof(json_data) == "string") {
            console.log(`plain text message received: ${json_data}`)
            return;
        }

        switch(json_data["type"]) {
            case "FriendRequest":
                handle_friend_request_update(json_data["request_id"], json_data["user"], json_data["status"]);
                break;
            case "LobbyUserListChange":
                handle_lobby_user_list_change(json_data["users"]);
                break;
            case "LobbyUserReadyChange":
                handle_lobby_user_ready_change(json_data["user"], json_data["ready"]);
                break;
            default:
                console.log(`Unrecognized message type: ${json_data["type"]}`);
                break;
        }
    }
}
