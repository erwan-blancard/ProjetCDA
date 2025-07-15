import { get_current_game_info } from '../api/account';
import { LobbyDTO, LobbyEntryDTO, LobbyInfoDTO } from '../api/dto';
import { LobbyList } from '../ui/lobby_list';
import { LobbyView } from '../ui/lobby';
import { ViewMgr } from '../ui/viewmgr';
import { api_url, login_guard } from '../utils';
import { create_lobby, current_lobby_set_ready_state, get_current_lobby, leave_current_lobby, list_lobbies } from '../api/lobby';
import { displayPopup, displayYesNo } from '../ui/popup';

import { get_friend_requests } from '../api/account.js';
import { get_my_account } from '../api/account.js';
import { initFriendPanel, displayPseudo } from '../ui/friend_panel.js';

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

const lobbyViewElement = document.getElementById("lobby");


/** @type LobbyList */
const lobbyList = document.getElementById("lobby-list");

lobbyList.lobbyJoinedCallback = (lobby) => {
    console.log("Lobby Joined: ", lobby);
    lobbyViewElement.update(lobby);
    viewMgr.setPrimaryView(currentlobbyview);
};


lobbyList.busyCallback = (busy) => {
    document.getElementById("select-lobby-back-button").disabled = busy;
};


document.getElementById("select-lobby-back-button").onclick = () => {
    viewMgr.setPrimaryView(mainview);
};


try {
    // check if lobby
    const lobbyDTO = await get_current_lobby();
    lobbyViewElement.update(lobbyDTO);
    if (lobbyDTO != null) {
        viewMgr.setPrimaryView(currentlobbyview);
    } else {
        viewMgr.setPrimaryView(mainview);
    }
} catch (error) {
    console.log(`Error getting current lobby: ${error.message}`);
}


document.getElementById("to-create-lobby").onclick = () => {
    lobbyPasswordInput.value = "";   // clear password
    viewMgr.setPrimaryView(createlobbyview);
};
document.getElementById("to-join-lobby").onclick = () => {
    viewMgr.setPrimaryView(selectlobbyview);

    // refresh list
    lobbyList.refreshPage();
};


const lobbyPasswordCheck = document.getElementById("lobby-private-check");
const lobbyPasswordInput = document.getElementById("lobby-password-input");
const createLobbyValidateButton = document.getElementById("create-lobby-validate-button");
const createLobbyBackButton = document.getElementById("create-lobby-back-button");
lobbyPasswordCheck.checked = false;
lobbyPasswordInput.disabled = true;

lobbyPasswordCheck.onclick = () => {
    lobbyPasswordInput.disabled = !lobbyPasswordCheck.checked;
};


createLobbyValidateButton.onclick = async () => {
    const use_password = lobbyPasswordCheck.checked;
    const password = lobbyPasswordInput.value;

    createLobbyValidateButton.disabled = true;
    createLobbyBackButton.disabled = true;

    // disable interactions with password inputs
    lobbyPasswordInput.disabled = true;
    lobbyPasswordCheck.disabled = true;

    const lobby = await create_lobby(use_password ? password : null, true);

    if (lobby) {
        lobbyViewElement.update(lobby);
        viewMgr.setPrimaryView(currentlobbyview);
    }

    createLobbyValidateButton.disabled = false;
    createLobbyBackButton.disabled = false;

    // re-enable interactions with password inputs
    lobbyPasswordInput.disabled = !lobbyPasswordCheck.checked;
    lobbyPasswordCheck.disabled = false;
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


function updateFriendButtonColor(requests) {
  const btnShowFriendList = document.getElementById('show-friend-list');
  if (!btnShowFriendList) return;
  btnShowFriendList.style.backgroundColor = 'white';
  btnShowFriendList.style.color = 'black';
  if (requests && requests.length > 0) {
    btnShowFriendList.style.backgroundColor = '#d6ff70'; // jaune-vert
    btnShowFriendList.style.color = '#5a5a00';
  }
}

async function refreshFriendButtonColor() {
  // On attend que le bouton soit dans le DOM
  let tries = 0;
  function tryUpdate() {
    const btn = document.getElementById('show-friend-list');
    if (btn) {
      get_friend_requests().then(updateFriendButtonColor);
    } else if (tries < 10) {
      tries++;
      setTimeout(tryUpdate, 200); // réessaye dans 200ms
    }
  }
  tryUpdate();
}

async function updateWelcomeBar() {
  const welcomeUser = document.getElementById('welcome-username');
  if (!welcomeUser) return;
  try {
    const account = await get_my_account();
    console.log('Compte récupéré via /account/profile :', account);
    if (account && account.username) {
      alert('Pseudo récupéré : ' + account.username);
      welcomeUser.textContent = account.username;
    } else {
      welcomeUser.textContent = '';
    }
  } catch (e) {
    welcomeUser.textContent = '';
  }
}

document.addEventListener('DOMContentLoaded', () => {
  initFriendPanel();
  displayPseudo('main-username');

  // Ajout panel paramètres
  let settingsPanel = document.getElementById('settings-panel');
  let settingsPanelBackdrop = document.getElementById('settings-panel-backdrop');
  if (!settingsPanel) {
    settingsPanel = document.createElement('div');
    settingsPanel.id = 'settings-panel';
    settingsPanel.style.display = 'none';
    settingsPanel.style.position = 'fixed';
    settingsPanel.style.top = '50%';
    settingsPanel.style.left = '50%';
    settingsPanel.style.transform = 'translate(-50%, -50%)';
    settingsPanel.style.background = 'white';
    settingsPanel.style.border = '1px solid #ccc';
    settingsPanel.style.borderRadius = '10px';
    settingsPanel.style.padding = '24px 32px';
    settingsPanel.style.zIndex = '100';
    settingsPanel.innerHTML = '<h2>Paramètres</h2><div style="margin-top:1em;">(À venir)</div><button id="close-settings-panel" class="styled" style="margin-top:2em;">Fermer</button>';
    document.body.appendChild(settingsPanel);
  }
  if (!settingsPanelBackdrop) {
    settingsPanelBackdrop = document.createElement('div');
    settingsPanelBackdrop.id = 'settings-panel-backdrop';
    settingsPanelBackdrop.style.display = 'none';
    settingsPanelBackdrop.style.position = 'fixed';
    settingsPanelBackdrop.style.top = '0';
    settingsPanelBackdrop.style.left = '0';
    settingsPanelBackdrop.style.right = '0';
    settingsPanelBackdrop.style.bottom = '0';
    settingsPanelBackdrop.style.width = '100vw';
    settingsPanelBackdrop.style.height = '100vh';
    settingsPanelBackdrop.style.background = 'rgba(0,0,0,0.3)';
    settingsPanelBackdrop.style.zIndex = '99';
    document.body.appendChild(settingsPanelBackdrop);
  }
  const paramBtn = document.getElementById('settings-btn');
  if (paramBtn) {
    paramBtn.addEventListener('click', () => {
      document.getElementById('settings-menu').style.display = 'none';
      settingsPanel.style.display = 'block';
      settingsPanelBackdrop.style.display = 'block';
    });
  }
  settingsPanelBackdrop.addEventListener('click', () => {
    settingsPanel.style.display = 'none';
    settingsPanelBackdrop.style.display = 'none';
  });
  settingsPanel.addEventListener('click', (e) => {
    if (e.target.id === 'close-settings-panel') {
      settingsPanel.style.display = 'none';
      settingsPanelBackdrop.style.display = 'none';
    }
  });

  // Gestion du menu paramètres/déconnexion
  const settingsBtn = document.getElementById('show-settings-menu');
  const settingsMenu = document.getElementById('settings-menu');
  const logoutBtn = document.getElementById('logout-btn');
  const paramBtn2 = document.getElementById('settings-btn'); // Renommé pour éviter la confusion
  if (paramBtn2) {
    paramBtn2.addEventListener('click', () => {
      settingsMenu.style.display = 'none';
      // Ouvre le panel paramètres (pour l'instant, alert)
      alert('Panel paramètres à implémenter !');
    });
  }
  if (logoutBtn) {
    logoutBtn.addEventListener('click', () => {
      document.cookie = 'token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      window.location.href = '/login.html';
    });
  }

  // Bouton déconnexion en bas à droite
  const logoutFab = document.getElementById('logout-fab');
  if (logoutFab) {
    logoutFab.addEventListener('click', () => {
      document.cookie = 'token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      window.location.href = '/login.html';
    });
  }

  // Bouton déconnexion latéral
  const logoutSide = document.getElementById('logout-side');
  if (logoutSide) {
    logoutSide.addEventListener('click', () => {
      document.cookie = 'token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
      window.location.href = '/login.html';
    });
  }

  // Panel menu burger (panel central)
  let burgerPanel = document.getElementById('burger-panel');
  let burgerPanelBackdrop = document.getElementById('burger-panel-backdrop');
  if (!burgerPanel) {
    burgerPanel = document.createElement('div');
    burgerPanel.id = 'burger-panel';
    burgerPanel.style.display = 'none';
    burgerPanel.style.position = 'fixed';
    burgerPanel.style.top = '50%';
    burgerPanel.style.left = '50%';
    burgerPanel.style.transform = 'translate(-50%, -50%)';
    burgerPanel.style.background = 'white';
    burgerPanel.style.border = '1px solid #ccc';
    burgerPanel.style.borderRadius = '10px';
    burgerPanel.style.padding = '32px 40px';
    burgerPanel.style.zIndex = '100';
    burgerPanel.style.display = 'none';
    burgerPanel.innerHTML = '<button id="close-burger-panel" class="styled" style="font-size: 1.2em;">Retour</button>';
    document.body.appendChild(burgerPanel);
  }
  if (!burgerPanelBackdrop) {
    burgerPanelBackdrop = document.createElement('div');
    burgerPanelBackdrop.id = 'burger-panel-backdrop';
    burgerPanelBackdrop.style.display = 'none';
    burgerPanelBackdrop.style.position = 'fixed';
    burgerPanelBackdrop.style.top = '0';
    burgerPanelBackdrop.style.left = '0';
    burgerPanelBackdrop.style.right = '0';
    burgerPanelBackdrop.style.bottom = '0';
    burgerPanelBackdrop.style.width = '100vw';
    burgerPanelBackdrop.style.height = '100vh';
    burgerPanelBackdrop.style.background = 'rgba(0,0,0,0.3)';
    burgerPanelBackdrop.style.zIndex = '99';
    document.body.appendChild(burgerPanelBackdrop);
  }
  const burgerBtn = document.getElementById('show-settings-menu');
  if (burgerBtn) {
    burgerBtn.addEventListener('click', () => {
      // Ferme le menu déroulant s'il est ouvert
      if (settingsMenu) settingsMenu.style.display = 'none';
      // Affiche le panel central
      burgerPanel.style.display = 'block';
      burgerPanelBackdrop.style.display = 'block';
      // S'assure que le z-index est bien supérieur
      burgerPanel.style.zIndex = '200';
      burgerPanelBackdrop.style.zIndex = '199';
    });
  }
  document.body.addEventListener('click', (e) => {
    if (e.target.id === 'close-burger-panel' || e.target.id === 'burger-panel-backdrop') {
      burgerPanel.style.display = 'none';
      burgerPanelBackdrop.style.display = 'none';
    }
  });
});
setTimeout(updateWelcomeBar, 1000); 

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
        let json_data = ev.data;

        try {
            json_data = JSON.parse(ev.data);
        } catch (error) {
            console.log(`Could not convert message to JSON: ${json_data}`)
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
                console.log(`Unrecognized message type: ${json_data["type"]}`);
                break;
        }
    }
}
