import { LobbyEntryDTO, LobbyInfoDTO } from "../api/dto";

const MAX_LOBBY_USERS = 6;


export class LobbyEntry extends HTMLElement {
    /** @type LobbyInfoDTO */
    lobbyInfo;
    userListElement;
    subElement;
    statusElement;
    joinButton;

    constructor(lobbyInfoDTO) {
        super();
        this.lobbyInfo = lobbyInfoDTO;
        this.className = "lobby-entry";

        this.userListElement = document.createElement("ul");
        
        this.subElement = document.createElement("div");
        this.subElement.className = "sub";
        
        this.statusElement = document.createElement("p");
        this.statusElement.className = "lobby-status";

        this.joinButton = document.createElement("button");
        this.joinButton.className = "styled";
        this.joinButton.onclick = () => { this.join_lobby() };

        const joinButtonSpan = document.createElement("span");
        joinButtonSpan.textContent = "Join";

        this.subElement.appendChild(this.statusElement);
        this.subElement.appendChild(this.joinButton);
        this.joinButton.appendChild(joinButtonSpan);

        this.appendChild(this.userListElement);
        this.appendChild(this.subElement);

        this.update(this.lobbyInfo);
    }

    join_lobby() {
        console.log("Join Lobby Action");
    }

    update(lobbyInfoDTO) {
        this.lobbyInfo = lobbyInfoDTO;
        
        // clear users
        for (let i = 0; i < this.userListElement.children.length; i++) {
            this.userListElement.removeChild(this.userListElement.children[i]);
        }
        // this.userListElement.children.forEach(child => {
        //     this.userListElement.removeChild(child);
        // });

        lobbyInfoDTO.users.forEach(user => {
            const userElement = document.createElement("li");
            userElement.textContent = user;
            this.userListElement.appendChild(userElement);
        });

        let status = `${lobbyInfoDTO.users.length}/${MAX_LOBBY_USERS}`;
        this.statusElement.textContent = status;

        this.joinButton.disabled = lobbyInfoDTO.users.length >= MAX_LOBBY_USERS;
        this.statusElement.className = (lobbyInfoDTO.password ? "locked" : "");
    }
}


// wraps LobbyEntry
export class LobbyListEntry extends HTMLElement {
    lobbyEntry;

    constructor(lobbyEntry) {
        super();
        this.lobbyEntry = lobbyEntry;
        this.appendChild(lobbyEntry);
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
        this.#nextPageButton.disabled = this.#currentPage+1 == this.#pageCount;
        this.#pageLabel.textContent = `${this.#currentPage+1}/${this.#pageCount}`;
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

    push(lobbyDTO) {
        const lobby_entry = new LobbyEntry(lobbyDTO);
        const lobby_list_entry = new LobbyListEntry(lobby_entry);
        this.#lobbyListEntries.push(lobby_list_entry);
        this.#lobbyListElement.appendChild(lobby_list_entry);
        
        this.#updateView();
    }

    remove_entry(lobbyDTO) {
        const index = this.indexOf(lobbyDTO);

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

    get_lobby_dto(index) {
        return this.#lobbyListEntries.at(index).lobbyEntry.lobby;
    }

    get_lobby_entry(index) {
        return this.#lobbyListEntries.at(index).lobbyEntry;
    }

    indexOf(lobbyDTO) {
        for (let i = 0; i < this.#lobbyListEntries.length; i++) {
            if (this.#lobbyListEntries.at(i).lobbyEntry.lobby == lobbyDTO) {
                return i;
            }
        }

        return -1;
    }

    refreshPage() {
        this.#busy = true;

        

        this.#busy = false;
    }

    prevPage() {
        if (!this.#busy) {
            this.#currentPage -= 1;

            this.refreshPage();

            this.#updateControls();
        }
    }

    nextPage() {
        if (!this.#busy) {
            this.#currentPage += 1;

            this.refreshPage();

            this.#updateControls();
        }
    }

}


// register elements
customElements.define("lobby-entry", LobbyEntry);
customElements.define("lobby-list-entry", LobbyListEntry);
customElements.define("lobby-list", LobbyList);
