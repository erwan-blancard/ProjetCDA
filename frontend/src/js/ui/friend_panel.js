import { get_friends, send_friend_request, get_friend_requests, respond_friend_request, delete_friend } from '../api/friends.js';
import { get_account } from '../api/account.js';
import { APP_STATE } from '../app_state.js';
import { strjoin } from '../utils.js';
import { FriendDTO, FriendWithLobbyStatusDTO } from '../api/dto.js';
import { join_lobby } from '../api/lobby.js';

const PANEL_TITLE_TEXT = "Friends list";
const SEND_REQUEST_LABEL = "Send friend request:";
const FRIEND_IN_LOBBY_TEXT = "In lobby";

const WAITING_STATUS_TEXT = "Waiting";

const REQUEST_ACCEPT_TEXT = "Accept";
const REQUEST_ACCEPTED_TEXT = "Accepted";
const REQUEST_DECLINE_TEXT = "Decline";
const REQUEST_DECLINED_TEXT = "Declined";
const REQUEST_CANCEL_TEXT = "Cancel";
const REQUEST_CLEAR_DECLINED_TEXT = "Clear";

const EMPTY_LIST_TEXT = "Nothing to show here...";


export class FriendRequestEntry extends HTMLElement {
    /** @type {FriendDTO | null} */
    info = null;
    username;
    // child elements
    label;
    acceptBtn;
    declineBtn;
    // only used if we are the sender
    cancelBtn;
    // only used for the declined requests pane (TODO)
    clearDeclinedBtn;

    constructor(friendDTO, username) {
        super();

        this.label = document.createElement("span");
        const btns = document.createElement("div");
        btns.className = "button-container";

        this.acceptBtn = document.createElement("button");
        this.acceptBtn.className = "accept-button";
        this.acceptBtn.textContent = REQUEST_ACCEPT_TEXT;
        this.declineBtn = document.createElement("button");
        this.declineBtn.className = "decline-button";
        this.declineBtn.textContent = REQUEST_DECLINE_TEXT;
        this.cancelBtn = document.createElement("button");
        this.cancelBtn.className = "cancel-button";
        this.cancelBtn.textContent = REQUEST_CANCEL_TEXT;
        this.clearDeclinedBtn = document.createElement("button");
        this.clearDeclinedBtn.className = "clear-declined-button";
        this.clearDeclinedBtn.textContent = REQUEST_CLEAR_DECLINED_TEXT;

        // onclick events are setup by FriendPanel

        btns.appendChild(this.acceptBtn);
        btns.appendChild(this.declineBtn);
        btns.appendChild(this.cancelBtn);
        btns.appendChild(this.clearDeclinedBtn);

        this.appendChild(this.label);
        this.appendChild(btns);
        
        this.update(friendDTO, username);
    }

    update(friendDTO, username) {
        if (friendDTO != undefined && username != undefined) {
            this.info = friendDTO;
            this.username = username;

            this.id = `friend-request-entry-${friendDTO.id}`;

            const user_is_sender = friendDTO.account1 == APP_STATE.account.id;
            this.acceptBtn.style.display = (friendDTO.status == 0 && !user_is_sender) ? "initial" : "none";
            this.declineBtn.style.display = (friendDTO.status == 0 && !user_is_sender) ? "initial" : "none";
            this.cancelBtn.style.display = (friendDTO.status == 0 && user_is_sender) ? "initial" : "none";
            this.clearDeclinedBtn.style.display = (friendDTO.status == 2 && !user_is_sender) ? "initial" : "none";

            let status = WAITING_STATUS_TEXT;
            if (friendDTO.status == 1) status = REQUEST_ACCEPTED_TEXT;
            else if (friendDTO.status == 2) status = REQUEST_DECLINED_TEXT;
            this.label.textContent = `${username} - ${status}`;
        }
    }
}


export class FriendEntry extends HTMLElement {
    /** @type {FriendWithLobbyStatusDTO | null} */
    info = null;
    label;
    removeBtn;
    joinLobbyBtn;

    constructor(friendWithLobbyStatusDTO) {
        super();

        this.label = document.createElement("span");
        this.label.className = "friend-entry-label";
        const btns = document.createElement("div");
        btns.className = "button-container";

        this.removeBtn = document.createElement("button");
        this.removeBtn.className = "remove-button";
        this.removeBtn.textContent = "X";
        this.joinLobbyBtn = document.createElement("button");
        this.joinLobbyBtn.className = "join-lobby-button";
        this.joinLobbyBtn.textContent = ">";

        this.appendChild(this.label);
        btns.appendChild(this.joinLobbyBtn);
        btns.appendChild(this.removeBtn);
        this.appendChild(btns);

        this.update(friendWithLobbyStatusDTO);
    }

    update(friendWithLobbyStatusDTO) {
        if (friendWithLobbyStatusDTO != undefined) {
            this.info = friendWithLobbyStatusDTO;

            this.label.textContent = friendWithLobbyStatusDTO.username;

            if (friendWithLobbyStatusDTO.lobby_id != null) {
                this.label.textContent += " - ";
                this.label.textContent += FRIEND_IN_LOBBY_TEXT;
                this.label.textContent += ` (${friendWithLobbyStatusDTO.lobby_id})`;
                this.joinLobbyBtn.style.display = "initial";
            } else {
                this.joinLobbyBtn.style.display = "none";
            }
        }
    }
}


export class FriendPanel extends HTMLElement {
    friends = [];
    requests = [];

    lobbyJoinedCallback = null;

    constructor() {
        super();

        this.closeBtn = document.createElement("i");
        this.closeBtn.className = "fas fa-times-circle";
        this.closeBtn.style.position = "absolute";
        this.closeBtn.style.top = "0.4em";
        this.closeBtn.style.right = "0.4em";
        this.closeBtn.style.cursor = "pointer";
        this.appendChild(this.closeBtn);

        const title = document.createElement("h3");
        title.textContent = PANEL_TITLE_TEXT;
        this.appendChild(title);

        this.friendRequestsDiv = document.createElement('div');
        this.friendRequestsDiv.className = 'friend-requests-list';
        this.appendChild(this.friendRequestsDiv);

        this.friendListDiv = document.createElement("div");
        this.friendListDiv.className = "friend-list";
        this.appendChild(this.friendListDiv);

        const sendRequestLabel = document.createElement("label");
        sendRequestLabel.textContent = SEND_REQUEST_LABEL;
        sendRequestLabel.setAttribute('for', 'add-friend-name');
        this.appendChild(sendRequestLabel);

        this.addFriendInput = document.createElement("input");
        this.addFriendInput.id = "add-friend-name";
        this.addFriendInput.type = "text";
        this.addFriendInput.placeholder = "Username";
        this.addFriendInput.maxLength = "32";
        this.appendChild(this.addFriendInput);

        // Buttons + feedback
        const btnContainer = document.createElement('div');

        this.addFriendButton = document.createElement('button');
        this.addFriendButton.className = 'styled';
        this.addFriendButton.innerHTML = '<span>Send Request</span>';
        btnContainer.appendChild(this.addFriendButton);

        this.appendChild(btnContainer);

        this.friendActionFeedback = document.createElement('div');
        this.friendActionFeedback.id = 'friend-action-feedback';
        this.appendChild(this.friendActionFeedback);

        this.addFriendButton.onclick = async () => { await this.onSendRequestClicked() };
    }

    async getFriendList() {
        // get friend list from API
        let apiFriends = [];
        try {
            const res = await get_friends();
            if (Array.isArray(res)) {
                apiFriends = res;
            }
        } catch (e) {
            console.error('Error querying friends list:', e);
        }
        
        return apiFriends;
    }

    async updateFriendList() {
        this.friendListDiv.innerHTML = '';
        this.friends = await this.getFriendList();

        const sorted = this.friends.sort((a, b) => {
            if (b.in_lobby !== a.in_lobby) return b.in_lobby - a.in_lobby;
            return a.username.localeCompare(b.username);
        });

        sorted.forEach((friend) => {
            const entry = this.createFriendEntry(friend);
            this.friendListDiv.appendChild(entry);
        });

        if (sorted.length == 0) {
            const empty_msg = document.createElement("span");
            empty_msg.className = "empty-list-label";
            empty_msg.textContent = EMPTY_LIST_TEXT;
            this.friendListDiv.appendChild(empty_msg);
        }

    }

    createFriendEntry(friend) {
        const entry = new FriendEntry(friend);

        entry.removeBtn.onclick = async () => {
            if (await delete_friend(entry.info.username) != null) {
                entry.remove();
            }
        };

        entry.joinLobbyBtn.onclick = async () => {
            if (this.lobbyJoinedCallback != null && entry.info.lobby_id) {

                // can't join if in lobby
                if (!APP_STATE.lobby) {

                    const lobby = await join_lobby(entry.info.lobby_id);
                    if (lobby) {
                        this.lobbyJoinedCallback(lobby);
                    } else {
                        this.friendActionFeedback.textContent = "Unable to join the lobby.";
                        this.friendActionFeedback.style.color = "red";
                    }

                } else {
                    this.friendActionFeedback.textContent = "Can't join: already in a lobby.";
                    this.friendActionFeedback.style.color = "orange";
                }
            }
        }

        return entry;
    }

    async getFriendRequests() {
        try {
            const requests = await get_friend_requests();
            if (!Array.isArray(requests))
                throw new Error("Unexpected response format !");
            return requests;
        } catch (e) {
            console.error('Error getting friend requests:', e);
            return [];
        }
    }

    async updateFriendRequests() {
        this.friendRequestsDiv.innerHTML = '';
        this.requests = await this.getFriendRequests();

        if (this.requests.length == 0) {
            return;
        }

        this.requests.forEach(async req => {
            const entry = await this.createFriendRequestEntry(req);
            this.friendRequestsDiv.appendChild(entry);
        });
    }

    /**
    * Request can either have been sent or received.
    * If received -> show accept + decline buttons.
    * If sent -> show cancel button.
    */
    async createFriendRequestEntry(req) {
        const account = await get_account(APP_STATE.account.id == req.account1 ? req.account2 : req.account1);
        const username = account ? account.username : "Unknown";

        const entry = new FriendRequestEntry(req, username);

        entry.acceptBtn.onclick = async () => {
            if (await respond_friend_request(entry.username, true) != null) {
                entry.remove();
                await this.updateFriendList();
            }
        };

        entry.declineBtn.onclick = async () => {
            const friend = await respond_friend_request(entry.username, false);
            if (friend != null) {
                entry.update(friend);
            }
        };

        entry.cancelBtn.onclick = async () => {
            // deletes the pending request
            if (await delete_friend(entry.username) != null) {
                entry.remove();
            }
        };

        entry.clearDeclinedBtn.onclick = async () => {
            // deletes the declined request, allowing the user to send a new request
            if (await delete_friend(entry.username) != null) {
                entry.remove();
            }
        };

        return entry;
    }

    async onSendRequestClicked() {
        const name = this.addFriendInput.value.trim();
        if (!name) {
            this.friendActionFeedback.textContent = "Please enter a valid name";
            this.friendActionFeedback.style.color = 'red';
            return;
        }

        // Check if not already in list
        const currentFriends = Array.from(this.friendListDiv.children).map(entry => entry.username);
        if (currentFriends.some(f => f === name)) {
            this.friendActionFeedback.textContent = "Already friend with this user.";
            this.friendActionFeedback.style.color = 'red';
            return;
        }

        this.friendActionFeedback.textContent = "Sending request...";
        this.friendActionFeedback.style.color = 'black';

        const result = await send_friend_request(name);
        if (result && result.id) {
            this.friendActionFeedback.textContent = `Request sent to ${name}`;
            this.friendActionFeedback.style.color = 'green';
            this.addFriendInput.value = '';

            // create entry
            const entry = await this.createFriendRequestEntry(result);
            this.friendListDiv.prepend(entry);
        } else {
            this.friendActionFeedback.textContent = "Error : " + (result && result.message ? result.message : "Could not send request with this username.");
            this.friendActionFeedback.style.color = 'red';
        }
    }

    async handleFriendRequestUpdate(request_id, user_id, status) {
        switch (status) {
            // waiting
            case 0: {
                    // "fake" req data
                    const req = { "id": request_id, "account1": user_id, "account2": APP_STATE.account.id, "status": status };
                    const entry = await this.createFriendRequestEntry(req);
                    this.friendRequestsDiv.prepend(entry);
                }
                break;
            // accepted
            case 1: {
                    const requestElement = document.getElementById(`friend-request-entry-${request_id}`);
                    if (requestElement) {
                        let username = "Unknown";
                        try {
                            // get username from request element based on text content
                            const sep = " - ";
                            const split = requestElement.firstElementChild.textContent.split(sep);
                            if (split.length > 1)
                                username = strjoin(split.splice(split.length-1), sep);
                        } catch (e) {
                            console.error(e);
                        }

                        requestElement.remove();

                        // "fake" friend data
                        // FIXME include username and lobby_id in sse data
                        const friend = { "id": request_id, "username": username, "lobby_id": null };
                        const entry = this.createFriendEntry(friend);
                        this.friendListDiv.prepend(entry);
                    } else {
                        await this.updateFriendList();
                    }
                    
                }
                break;
            // rejected
            case 2: {
                    const requestElement = document.getElementById(`friend-request-entry-${request_id}`);
                    if (requestElement)
                        requestElement.remove();
                }
                break;
        }
    }

    async handleFriendshipDeleted(request_id) {
        // could be from a canceled request
        const requestElement = document.getElementById(`friend-request-entry-${request_id}`);
        if (requestElement)
            requestElement.remove();

        const friendElement = document.getElementById(`friend-entry-${request_id}`);
        if (friendElement)
            friendElement.remove();
    }
}


customElements.define("friend-panel", FriendPanel);
customElements.define("friend-entry", FriendEntry);
customElements.define("friend-request-entry", FriendRequestEntry);
