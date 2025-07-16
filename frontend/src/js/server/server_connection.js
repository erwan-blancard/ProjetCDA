import { ChangeTurnResponse, CollectDiscardCardsResponse, DrawCardResponse, GameEndResponse, GameStatusResponse, MessageResponse, PlayCardResponse, PlayerBuffStatusResponse, SessionInfoResponse } from "./dto"

// these types must match the UserActions in the gameserver
export const PLAY_CARD_ACTION_KEY = "PlayCard"
export const DRAW_CARD_ACTION_KEY = "DrawCard"
export const SEND_CHAT_MESSAGE_ACTION_KEY = "SendChatMessage"

export const CHAT_MESSAGE_RESP_KEY = "Message"
export const GAME_STATUS_RESP_KEY = "GameStatus"
export const SESSION_INFO_RESP_KEY = "SessionInfo"
export const PLAY_CARD_RESP_KEY = "PlayCard"
export const DRAW_CARD_RESP_KEY = "DrawCard"
export const CHANGE_TURN_RESP_KEY = "ChangeTurn"
export const PLAYER_BUFF_STATUS_RESP_KEY = "PlayerBuffStatus"
export const COLLECT_DISCARD_CARDS_RESP_KEY = "CollectDiscardCards"
export const GAME_END_RESP_KEY = "GameEnd"


export class ServerConnexion extends EventTarget {
    /** @type {WebSocket | null} */
    #socket = null
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

    connect(wsUrl) {
        this.#socket = new WebSocket(wsUrl);

        this.#socket.onerror = ev => {
            console.log("WebSocket error:", ev);
        }

        // send token when opened
        this.#socket.onopen = () => {
            this.#emitConnectionChangeEvent();
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

    #process_received_data(data) {
        console.log("Data received: " + data);

        const json_data = JSON.parse(data);
        
        const resp_type = json_data["type"];
        switch (resp_type) {
            case CHAT_MESSAGE_RESP_KEY:
                this.dispatchEvent(new CustomEvent("chatmessage", { detail:
                    new MessageResponse(json_data)
                }))
                break;
        
            case GAME_STATUS_RESP_KEY:
                this.dispatchEvent(new CustomEvent("gameupdate", { detail: 
                    new GameStatusResponse(json_data)
                }))
                break;
        
            case SESSION_INFO_RESP_KEY:
                this.#session_info = { id: json_data["id"], players: json_data["players"] };
                this.dispatchEvent(new CustomEvent("sessioninfo", { detail:
                    new SessionInfoResponse(json_data)
                }))
                break;
            
            case PLAY_CARD_RESP_KEY:
                this.dispatchEvent(new CustomEvent("playcard", { detail:
                    new PlayCardResponse(json_data)
                }))
                break;
            
            case DRAW_CARD_RESP_KEY:
                this.dispatchEvent(new CustomEvent("drawcard", { detail:
                    new DrawCardResponse(json_data)
                }))
                break;
            
            case CHANGE_TURN_RESP_KEY:
                this.dispatchEvent(new CustomEvent("changeturn", { detail:
                    new ChangeTurnResponse(json_data)
                }))
                break;
            
            case PLAYER_BUFF_STATUS_RESP_KEY:
                this.dispatchEvent(new CustomEvent("playerbuffstatus", { detail:
                    new PlayerBuffStatusResponse(json_data)
                }))
                break;
        
            case COLLECT_DISCARD_CARDS_RESP_KEY:
                this.dispatchEvent(new CustomEvent("collectdiscardcards", { detail:
                    new CollectDiscardCardsResponse(json_data)
                }))
                break;
            
            case GAME_END_RESP_KEY:
                this.dispatchEvent(new CustomEvent("gameend", { detail:
                    new GameEndResponse(json_data)
                }))
                break;

            default:
                console.log("Unrecognized response type:", resp_type)
                break;
        }
    }

    // events

    #emitConnectionChangeEvent() {
        this.dispatchEvent(new CustomEvent("connectionchange", { detail:
            { status: this.is_connected() }
        }))
    }

    // actions

    send_chat_message(message) {
        const action = {
            "type": SEND_CHAT_MESSAGE_ACTION_KEY,
            "message": message
        };
        this.#socket.send(JSON.stringify(action));
    }

    send_play_card_action(card_index, targets) {
        const action = {
            "type": PLAY_CARD_ACTION_KEY,
            "card_index": card_index,
            "targets": targets
        };
        this.#socket.send(JSON.stringify(action));
    }

    send_draw_card_action() {
        const action = {
            "type": DRAW_CARD_ACTION_KEY
        };
        this.#socket.send(JSON.stringify(action));
    }
}