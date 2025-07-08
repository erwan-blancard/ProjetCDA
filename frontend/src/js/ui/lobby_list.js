import { get_account } from "../api/account";
import { LobbyEntryDTO, LobbyInfoDTO } from "../api/dto";
import { join_lobby, list_lobbies } from "../api/lobby";
import { displayInput } from "./popup";

const MAX_LOBBY_USERS = 6;


export class LobbyEntry extends HTMLElement {
    /** @type LobbyEntryDTO */
    lobbyEntry;
    userListElement;
    subElement;
    statusElement;
    joinButton;

    lobbyJoinedCallback = null;

    constructor(lobbyEntryDTO) {
        super();
        this.lobbyEntry = lobbyEntryDTO;
        this.className = "lobby-entry";

        this.userListElement = document.createElement("ul");
        
        this.subElement = document.createElement("div");
        this.subElement.className = "sub";
        
        this.statusElement = document.createElement("p");
        this.statusElement.className = "lobby-status";

        this.joinButton = document.createElement("button");
        this.joinButton.className = "styled";
        this.joinButton.onclick = () => { this.on_join_clicked() };

        const joinButtonSpan = document.createElement("span");
        joinButtonSpan.textContent = "Join";

        this.subElement.appendChild(this.statusElement);
        this.subElement.appendChild(this.joinButton);
        this.joinButton.appendChild(joinButtonSpan);

        this.appendChild(this.userListElement);
        this.appendChild(this.subElement);

        this.update(this.lobbyEntry);
    }

    async on_join_clicked() {
        if (this.lobbyJoinedCallback != null) {

            this.joinButton.disabled = true;
            if (this.lobbyEntry.password) {
                displayInput("Password:", "Entry Lobby Password", "Join", true, async (inputElement) => {

                    const lobby = await join_lobby(this.lobbyEntry.lobby_id, inputElement.value);
                    if (lobby != null) {
                        this.lobbyJoinedCallback(lobby);
                    } else {
                        this.joinButton.disabled = false;
                    }

                });
            } else {
                const lobby = await join_lobby(this.lobbyEntry.lobby_id);
                if (lobby != null) {
                    this.lobbyJoinedCallback(lobby);
                } else {
                    this.joinButton.disabled = false;
                }
            }
        }
    }

    update(lobbyEntryDTO) {
        this.lobbyEntry = lobbyEntryDTO;
        
        // clear users
        for (let i = this.userListElement.children.length-1; i >= 0; i--) {
            this.userListElement.removeChild(this.userListElement.children[i]);
        }

        lobbyEntryDTO.users.forEach(async (user_id) => {
            const userElement = document.createElement("li");
            const accountDTO = await get_account(user_id);
            userElement.textContent = accountDTO.username;
            this.userListElement.appendChild(userElement);
        });

        let status = `${lobbyEntryDTO.users.length}/${MAX_LOBBY_USERS}`;
        this.statusElement.textContent = status;

        this.joinButton.disabled = lobbyEntryDTO.users.length >= MAX_LOBBY_USERS;
        this.statusElement.className = (lobbyEntryDTO.password ? "locked" : "");
    }
}


export class LobbyList extends HTMLElement {
    #lobbyListEntries = [];
    #lobbyListElement;
    #pageLabel;
    #prevPageButton;
    #nextPageButton;
    #listStatusLabel;

    #currentPage = 0;
    #pageCount = 1;
    #busy = false;

    lobbyJoinedCallback = null;

    constructor() {
        super();
        this.className = "lobby-list";

        const controlDiv = document.createElement("div");
        controlDiv.className = "lobby-list-controls";

        this.#prevPageButton = document.createElement("button");
        this.#prevPageButton.textContent = "<";
        this.#prevPageButton.onclick = () => { this.prevPage(); }
        this.#nextPageButton = document.createElement("button");
        this.#nextPageButton.textContent = ">";
        this.#nextPageButton.onclick = () => { this.nextPage(); }

        this.#pageLabel = document.createElement("span");

        controlDiv.appendChild(this.#prevPageButton);
        controlDiv.appendChild(this.#pageLabel);
        controlDiv.appendChild(this.#nextPageButton);
        this.appendChild(controlDiv);

        this.#listStatusLabel = document.createElement("div");
        this.#listStatusLabel.className = "lobby-list-status";
        this.appendChild(this.#listStatusLabel);

        this.#lobbyListElement = document.createElement("div");
        this.#lobbyListElement.className = "lobby-list-container";
        this.appendChild(this.#lobbyListElement);

        this.#updateControls();
        this.#updateView();
    }

    #updateControls() {
        this.#prevPageButton.disabled = this.#currentPage-1 < 0;
        this.#nextPageButton.disabled = this.#currentPage+1 >= this.#pageCount;
        this.#pageLabel.textContent = `${(this.#pageCount > 0 ? this.#currentPage+1 : 0)}/${this.#pageCount}`;
    }

    #updateView() {
        if (this.#lobbyListEntries.length > 0) {
            this.#listStatusLabel.style.display = "none";
            this.#lobbyListElement.style.display = "inherit";
        } else {
            if (this.#busy)
                this.#listStatusLabel.textContent = "Updating...";
            else
                this.#listStatusLabel.textContent = "Nothing to show";
            this.#listStatusLabel.style.display = "inherit";
            this.#lobbyListElement.style.display = "none";
        }
    }

    count() { return this.#lobbyListEntries.length; }

    clear() {
        let entry = this.#lobbyListEntries.pop();
        while (entry != null) {
            this.#lobbyListElement.removeChild(entry);
            entry = this.#lobbyListEntries.pop();
        }
        
        this.#updateView();
    }
    
    /** push lobby entry object and creates the necessary HTML elements
    to display it on the list */
    push(lobbyEntryDTO) {
        const lobby_entry = new LobbyEntry(lobbyEntryDTO);
        lobby_entry.lobbyJoinedCallback = this.lobbyJoinedCallback;
        this.#lobbyListEntries.push(lobby_entry);
        this.#lobbyListElement.appendChild(lobby_entry);
        
        this.#updateView();
    }

    remove_entry(lobbyEntryDTO) {
        const index = this.indexOf(lobbyEntryDTO);

        if (index != -1) {
            const entry = this.#lobbyListEntries.splice(index, 1)[0];
            this.#lobbyListElement.removeChild(entry);
        }
        
        this.#updateView();
    }

    remove_index(index) {
        const entry = this.#lobbyListEntries.splice(index, 1)[0];
        this.#lobbyListElement.removeChild(entry);
        this.#updateView();
    }

    indexOf(lobbyEntryDTO) {
        for (let i = 0; i < this.#lobbyListEntries.length; i++) {
            if (this.#lobbyListEntries.at(i).lobby == lobbyEntryDTO) {
                return i;
            }
        }

        return -1;
    }

    async refreshPage() {
        this.#busy = true;
        this.clear();
        this.#updateControls();

        const lobbies_info = await list_lobbies(this.#currentPage);

        if (lobbies_info) {
            this.#pageCount = lobbies_info.page_count;
            // this.#currentPage = lobbies_info.page;
            
            lobbies_info.entries.forEach(lobby_entry => {
                this.push(lobby_entry);
            });
        }

        this.#busy = false;
        this.#updateControls();
    }

    prevPage() {
        if (!this.#busy) {
            this.#currentPage -= 1;

            this.refreshPage();
        }
    }

    nextPage() {
        if (!this.#busy) {
            this.#currentPage += 1;

            this.refreshPage();
        }
    }

}


// register elements
customElements.define("lobby-entry", LobbyEntry);
customElements.define("lobby-list", LobbyList);
