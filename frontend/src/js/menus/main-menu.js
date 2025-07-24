import { get_current_game_info } from '../api/account';
import { LobbyList } from '../ui/lobby_list';
import { ViewMgr } from '../ui/viewmgr';
import { api_url, login_guard } from '../utils';
import { create_lobby, current_lobby_set_ready_state, get_current_lobby, leave_current_lobby } from '../api/lobby';
import { displayInput, displayPopup, displayYesNo } from '../ui/popup';
import { LobbyView } from '../ui/lobby';
import { FriendPanel } from '../ui/friend_panel';
import { APP_STATE } from '../app_state';


// debug
window.APP_STATE = APP_STATE;


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

// callback to use when a lobby is joined successfully
function lobbyJoinedCallback(lobby) {
    console.log("Lobby Joined: ", lobby);
    APP_STATE.lobby = lobby;
    lobbyViewIDElement.textContent = lobby.id;
    lobbyViewElement.update(lobby);
    viewMgr.setPrimaryView(currentlobbyview);
}

lobbyList.lobbyJoinedCallback = lobbyJoinedCallback;


lobbyList.busyCallback = (busy) => {
    document.getElementById("select-lobby-back-button").disabled = busy;
    document.getElementById("lobby-direct-join").disabled = busy;
};

document.getElementById("select-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};

document.getElementById("lobby-direct-join").onclick = async () => {
    const lobby_id = await displayInput("Code:", "Enter lobby code", "Join", {"maxLength": "7"}, {"textTransform": "uppercase", "width": "7em"});

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
        APP_STATE.lobby = lobbyDTO;
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


// Friends panel elements
/** @type {FriendPanel} */
const friendPanel = document.getElementById('friend-panel');
const btnShowFriendList = document.getElementById('show-friend-list');
const friendPanelBackdrop = document.getElementById('friend-panel-backdrop');
const closeFriendButton = document.getElementById('close-friend-button');

friendPanel.lobbyJoinedCallback = lobbyJoinedCallback;

function showFriendPanel() {
    friendPanel.friendActionFeedback.textContent = "";
    friendPanel.style.display = 'block';
    friendPanelBackdrop.style.display = 'block';

    // disable interactions for views
    viewMgr.setInert();

    updateShowPanelButtonColor(false);
}

function closeFriendPanel() {
    friendPanel.style.display = 'none';
    friendPanelBackdrop.style.display = 'none';

    // re-enable interactions for views
    viewMgr.removeInert();
}

function updateShowPanelButtonColor(pending_updates=true) {
    if (pending_updates) {
        btnShowFriendList.style.backgroundColor = "#f0c115";
        // update color only if not already open
        if (friendPanel.style.display == "none") {
            btnShowFriendList.style.backgroundColor = "#f0c115";
        }
    } else {
        btnShowFriendList.style.backgroundColor = "white";
    }
}

btnShowFriendList.addEventListener('click', () => {
  const isHidden = friendPanel.style.display === 'none' || friendPanel.style.display === '';
  if (isHidden) {
    friendPanel.updateFriendRequests();
    friendPanel.updateFriendList();

    // Ajoute la section demandes d'amis en haut du panneau si pas déjà présente
    if (!friendPanel.contains(friendPanel.friendRequestsDiv)) {
      friendPanel.insertBefore(friendPanel.friendRequestsDiv, friendPanel.firstChild);
      // Optionnel: titre
      if (!document.getElementById('friend-requests-title')) {
        const title = document.createElement('div');
        title.id = 'friend-requests-title';
        title.textContent = 'Demandes d\'amis reçues';
        title.style.fontWeight = 'bold';
        title.style.marginBottom = '5px';
        friendPanel.insertBefore(title, friendPanel.friendRequestsDiv);
      }
    }
    showFriendPanel();
  } else {
    closeFriendPanel();
  }
});

closeFriendButton.addEventListener('click', closeFriendPanel);
friendPanelBackdrop.addEventListener('click', closeFriendPanel);


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
        if (await leave_current_lobby()) {
            APP_STATE.lobby = null;
            viewMgr.setPrimaryView(mainview);
        } else {
            displayPopup("An error occured when leaving the lobby !", "Error !");
        }
        lobbyViewElement.leaveButton.disabled = false;
        lobbyViewElement.readyButton.disabled = false;
    });
};


function handle_friend_request_update(request_id, user_id, status) {
    console.log(`request_id: ${request_id}, user_id: ${user_id}, status: ${status}`);
    friendPanel.handleFriendRequestUpdate(request_id, user_id, status);
    updateShowPanelButtonColor();
}

function handle_friendship_deleted(request_id) {
    console.log(`request_id: ${request_id}`);
    friendPanel.handleFriendshipDeleted(request_id);
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

        console.log("Sse Message Type: "+json_data["type"]);

        switch(json_data["type"]) {
            case "FriendRequest":
                handle_friend_request_update(json_data["request_id"], json_data["user"], json_data["status"]);
                break;
            case "FriendshipDeleted":
                handle_friendship_deleted(json_data["id"]);
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
