import { get_current_game_info } from '../api/account';
import { LobbyList } from '../ui/lobby_list';
import { ViewMgr } from '../ui/viewmgr';
import { api_url, login_guard } from '../utils';
import { create_lobby, current_lobby_set_ready_state, get_current_lobby, leave_current_lobby } from '../api/lobby';
import { displayInput, displayPopup, displayYesNo } from '../ui/popup';
import { LobbyView } from '../ui/lobby';


const account = await login_guard();

try {
    // check if a game is running
    if (await get_current_game_info() != null) {
        window.location.href = "/ingame.html";
    }
} catch (error) {
    console.log(`Error getting current game: ${error.message}`);
}

const mainview = document.getElementById("main");
const createlobbyview = document.getElementById("create-lobby");
const selectlobbyview = document.getElementById("select-lobby");
const currentlobbyview = document.getElementById("current-lobby");

const viewMgr = new ViewMgr();

/** @type {LobbyView} */
const lobbyViewElement = document.getElementById("lobby");
const lobbyViewIDElement = document.getElementById("current-lobby-id")


/** @type {LobbyList} */
const lobbyList = document.getElementById("lobby-list");

lobbyList.lobbyJoinedCallback = (lobby) => {
    console.log("Lobby Joined: ", lobby);
    lobbyViewIDElement.textContent = lobby.id;
    lobbyViewElement.update(lobby);
    viewMgr.setPrimaryView(currentlobbyview);
};


lobbyList.busyCallback = (busy) => {
    document.getElementById("select-lobby-back-button").disabled = busy;
    document.getElementById("lobby-direct-join").disabled = busy;
};

document.getElementById("select-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};

document.getElementById("lobby-direct-join").onclick = async () => {
    const lobby_id = await displayInput("Code:", "Enter lobby code", "Join", {"maxlength": "7"}, {"textTransform": "uppercase"});

    if (lobby_id) {
        // calls lobbyJoinedCallback()
        await lobbyList.joinWithCode(lobby_id);
    }
};


try {
    // check if lobby
    const lobbyDTO = await get_current_lobby();
    lobbyViewIDElement.textContent = lobbyDTO.id;
    lobbyViewElement.update(lobbyDTO);
    if (lobbyDTO != null) {
        viewMgr.setPrimaryView(currentlobbyview);
    } else {
        viewMgr.setPrimaryView(mainview);
    }
} catch (error) {
    console.log(`Error getting current lobby: ${error.message}`);
    viewMgr.setPrimaryView(mainview);
}


document.getElementById("to-create-lobby").onclick = () => {
    // reset create form inputs
    lobbyUnlistedCheck.checked = false;
    viewMgr.setPrimaryView(createlobbyview);
};
document.getElementById("to-join-lobby").onclick = () => {
    viewMgr.setPrimaryView(selectlobbyview);

    // refresh list
    lobbyList.refreshPage();
};


const createLobbyForm = document.getElementById("create-lobby-form");
const lobbyUnlistedCheck = document.getElementById("lobby-unlisted-check");
const createLobbyValidateButton = document.getElementById("create-lobby-validate-button");
const createLobbyBackButton = document.getElementById("create-lobby-back-button");


createLobbyValidateButton.onclick = async () => {
    const is_unlisted = lobbyUnlistedCheck.checked;

    createLobbyValidateButton.disabled = true;
    createLobbyBackButton.disabled = true;
    // disable form interactions
    createLobbyForm.disabled = true;

    const lobby = await create_lobby(is_unlisted);

    if (lobby) {
        lobbyViewIDElement.textContent = lobby.id;
        lobbyViewElement.update(lobby);
        viewMgr.setPrimaryView(currentlobbyview);
    }

    createLobbyValidateButton.disabled = false;
    createLobbyBackButton.disabled = false;
    // re-enable form interactions
    createLobbyForm.disabled = false;
};
createLobbyBackButton.onclick = () => {
    viewMgr.setPrimaryView(mainview);
};


lobbyViewElement.readyButton.onclick = async () => {
    if (lobbyViewElement.lobbyDTO != null) {
        const was_ready = lobbyViewElement.lobbyDTO.is_user_ready(account.id);
        lobbyViewElement.leaveButton.disabled = true;
        lobbyViewElement.readyButton.disabled = true;

        if (await current_lobby_set_ready_state(!was_ready)) {
            lobbyViewElement.update_user_ready_state(account.id, !was_ready)
        } else {
            displayPopup("An error occured trying to change ready state !", "Error !");
        }

        lobbyViewElement.leaveButton.disabled = false;
        lobbyViewElement.readyButton.disabled = false;
    }
};

lobbyViewElement.leaveButton.onclick = () => {
    displayYesNo("Leave this lobby ?", "", async () => {
        lobbyViewElement.leaveButton.disabled = true;
        lobbyViewElement.readyButton.disabled = true;
        if (await leave_current_lobby())
            viewMgr.setPrimaryView(mainview);
        else
            displayPopup("An error occured when leaving the lobby !", "Error !");
        lobbyViewElement.leaveButton.disabled = false;
        lobbyViewElement.readyButton.disabled = false;
    });
};


function handle_friend_request_update(request_id, user_id, status) {
    console.log(`request_id: ${request_id}, user_id: ${user_id}, status: ${status}`);
}

function handle_lobby_user_list_change(user_ids) {
    console.log(`user_ids: ${user_ids}`);
    lobbyViewElement.update_user_list(user_ids);
}

function handle_lobby_user_ready_change(user_id, ready) {
    console.log(`user_id: ${user_id}, ready: ${ready}`);
    lobbyViewElement.update_user_ready_state(user_id, ready);
}


let events;

if (account != null) {
    events = new EventSource(api_url("/events"), { withCredentials: true });

    events.onopen = () => {
        console.log("Listening sse...");
    }

    events.onerror = (ev) => {
        console.error("SSE Error:", ev);
    };

    events.onmessage = (ev) => {
        let json_data = ev.data;

        try {
            json_data = JSON.parse(ev.data);
        } catch (error) {
            console.error(`SSE: Could not convert message to JSON: ${json_data}`)
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
            case "GameStarted":
                window.location.href = "/ingame.html";
                break;
            default:
                console.error(`SSE: Unrecognized message type: ${json_data["type"]}`);
                break;
        }
    }
}
