import { get_account } from "../api/account";
import { LobbyDTO } from "../api/dto";


export class LobbyView extends HTMLElement {
    /** @type {LobbyDTO | null} */
    lobbyDTO = null;
    userListElement;
    subElement;
    readyButton;
    leaveButton;

    constructor() {
        super();
        this.className = "lobby-view";

        this.userListElement = document.createElement("ul");
        
        this.subElement = document.createElement("div");
        this.subElement.className = "sub";

        this.readyButton = document.createElement("button");
        this.readyButton.className = "styled";
        const readyButtonSpan = document.createElement("span");
        readyButtonSpan.textContent = "Ready !";

        this.leaveButton = document.createElement("button");
        this.leaveButton.className = "styled";
        const leaveButtonSpan = document.createElement("span");
        leaveButtonSpan.textContent = "Leave";

        this.subElement.appendChild(this.readyButton);
        this.readyButton.appendChild(readyButtonSpan);
        this.subElement.appendChild(this.leaveButton);
        this.leaveButton.appendChild(leaveButtonSpan);

        this.appendChild(this.userListElement);
        this.appendChild(this.subElement);

        this.update(null);
    }

    update(lobbyDTO) {
        this.lobbyDTO = lobbyDTO;

        // clear users
        for (let i = this.userListElement.children.length-1; i >= 0; i--) {
            this.userListElement.removeChild(this.userListElement.children[i]);
        }

        if (this.lobbyDTO != null) {
            this.lobbyDTO.users.forEach(async (user_id) => {
                const userElement = document.createElement("li");
                userElement.className = this.lobbyDTO.users_ready.indexOf(user_id) != -1 ? "ready" : "not-ready";
                const accountDTO = await get_account(user_id);
                if (accountDTO)
                    userElement.textContent = accountDTO.username;
                else
                    userElement.textContent = user_id;
                this.userListElement.appendChild(userElement);
            });
        }
    }

    update_user_ready_state(user_id, ready) {
        const idx = this.lobbyDTO.users_ready.indexOf(user_id);

        if (ready && idx == -1) {
            this.lobbyDTO.users_ready.push(user_id);
        } else if (!ready && idx != -1) {
            this.lobbyDTO.users_ready.splice(idx, 1);
        }

        this.update(this.lobbyDTO);
    }

    update_user_list(user_ids) {
        this.lobbyDTO.users = user_ids;
        this.update(this.lobbyDTO);
    }
}

// register element
customElements.define("lobby-view", LobbyView);
