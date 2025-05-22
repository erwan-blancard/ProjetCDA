export const AUTH_ACTION_KEY = "Auth"
export const PLAY_CARD_ACTION_KEY = "PlayCard"
export const DRAW_CARD_ACTION_KEY = "DrawCard"
export const SEND_CHAT_MESSAGE_ACTION_KEY = "SendChatMessage"

export const AUTH_RESP_KEY = "Auth"
export const CHAT_MESSAGE_RESP_KEY = "Message"
export const GAME_STATUS_RESP_KEY = "GameStatus"
export const SESSION_INFO_RESP_KEY = "SessionInfo"


export class ServerConnexion extends EventTarget {
    /** @type {WebSocket | null} */
    #socket = null
    #authenticated = false
    /**
     * @typedef {{
     *  id: any
     *  name: string
     * }}
     * PlayerProfile
     * 
     * @typedef {{
     *  id: any
     *  players: Array<PlayerProfile>
     * }}
     * SessionInfo
     * 
     * @type {SessionInfo | null}
     */
    #session_info = null;

    constructor() {
        super();
    }

    connect(wsUri, token) {
        this.#socket = new WebSocket(wsUri);

        this.#socket.onerror = ev => {
            console.log("WebSocket error:", ev);
        }

        // send token when opened
        this.#socket.onopen = () => {
            this.#emitConnectionChangeEvent();
            this.#send_auth_action(token);
        }

        this.onclose = () => {
            this.#socket = null;
            this.#emitConnectionChangeEvent();
        }

        this.#socket.onmessage = msg_event => {
            this.#process_received_data(msg_event.data);
        }
    }

    disconnect() {
        if (this.#socket) {
            this.#socket.close();
            this.#socket = null;
        }
        this.#session_info = null;
        this.#emitConnectionChangeEvent();
    }

    is_connected() {
        return this.#socket != null;
    }

    is_authenticated() {
        return this.#authenticated;
    }

    send_chat_message(message) {
        if (this.is_connected() && this.is_authenticated()) {
            this.#send_message_action(message);
        }
    }

    #process_received_data(data) {
        console.log("Data received: " + data);

        const json_data = JSON.parse(data);
        
        const resp_type = json_data["type"];
        switch (resp_type) {
            case AUTH_RESP_KEY:
                this.#authenticated = json_data["status"] == true;
                this.#emitAuthStatusChange();
                break;
        
            case CHAT_MESSAGE_RESP_KEY:
                this.#emitNewChatMessageEvent(json_data["message"]);
                break;
        
            case GAME_STATUS_RESP_KEY:
                this.#emitGameStateUpdateEvent(json_data["state"]);
                break;
        
            case SESSION_INFO_RESP_KEY:
                this.#session_info = { id: json_data["id"], players: json_data["players"] };
                this.#emitSessionInfo();
                break;
        
            default:
                console.log("Unrecognized response type:", resp_type)
                break;
        }
    }

    // events

    #emitGameStateUpdateEvent(json_data) {
        this.dispatchEvent(new CustomEvent("gameupdate", { detail: 
            json_data
         }))
    }

    #emitNewChatMessageEvent(message) {
        this.dispatchEvent(new CustomEvent("chatmessage", { detail:
            { message: message }
        }))
    }

    #emitConnectionChangeEvent() {
        this.dispatchEvent(new CustomEvent("connectionchange", { detail:
            { status: this.is_connected() }
        }))
    }

    #emitAuthStatusChange() {
        this.dispatchEvent(new CustomEvent("authchange", { detail:
            { status: this.#authenticated }
        }))
    }

    #emitSessionInfo() {
        this.dispatchEvent(new CustomEvent("sessioninfo", { detail:
            this.#session_info
        }))
    }

    // actions

    #send_auth_action(token) {
        const action = {
            "type": AUTH_ACTION_KEY,
            "token": token
        };
        this.#socket.send(JSON.stringify(action));
    }

    #send_message_action(message) {
        const action = {
            "type": SEND_CHAT_MESSAGE_ACTION_KEY,
            "message": message
        };
        this.#socket.send(JSON.stringify(action));
    }

    #send_play_card_action(card_id) {
        const action = {
            "type": PLAY_CARD_ACTION_KEY,
            "card_id": card_id
        };
        this.#socket.send(JSON.stringify(action));
    }

    #send_draw_card_action() {
        const action = {
            "type": DRAW_CARD_ACTION_KEY
        };
        this.#socket.send(JSON.stringify(action));
    }
}