import gsap from "gsap";
import { login_guard, ws_url } from "./utils";

import * as THREE from 'three';
import * as GAME from './game/game';
import { get_current_game_info } from "./api/account";
import { ChangeTurnResponse, DrawCardResponse, GameStatusResponse, SessionInfoResponse } from "./server/dto";
import { GameSessionInfoDTO } from "./api/dto";
import { CARD_DATABASE } from "./game/database";
import { randInt } from "three/src/math/MathUtils";
import { displayMessage } from "./ui/popup";


// expose for debug
window.GAME = GAME;


screen.orientation.lock("landscape");


// const account = await login_guard();

// const game_info = await get_current_game_info();

const game_info = new GameSessionInfoDTO({"game_id": 0, "players": [
    {"id": 0, "name": "Player 1"},
    {"id": 1, "name": "Player 2"},
    {"id": 2, "name": "Player 3"}
]});

const session_info = new SessionInfoResponse({"id": 0, "players": [
    {"id": 0, "name": "Player 1"},
    {"id": 1, "name": "Player 2"},
    {"id": 2, "name": "Player 3"}
]});

if (game_info != null) {
    const wsUrl = ws_url(game_info.game_id);

    try {
        GAME.initGame();
    } catch (e) {
        displayMessage(`Error when initializing Game: ${e.message}`);
    }

    document.getElementById("simulate-current-turn").onclick = () => {
        GAME.onChangeTurnEvent(new ChangeTurnResponse({"player_id": 0}));
    }

    document.getElementById("simulate-opponent-turn").onclick = () => {
        GAME.onChangeTurnEvent(new ChangeTurnResponse({"player_id": 1}));
    }

    document.getElementById("simulate-draw-card").onclick = () => {
        GAME.onDrawCardEvent(new DrawCardResponse({"player_id": 0, "card_id": randInt(0, CARD_DATABASE.size-1)}));
    }

    document.getElementById("simulate-draw-card-opponent").onclick = () => {
        GAME.onDrawCardEvent(new DrawCardResponse({"player_id": 1, "card_id": -1}));
    }

    document.getElementById("simulate-game-update").onclick = () => {
        GAME.onServerUpdate(new GameStatusResponse({
            "current_player_turn": 0,
            "current_player_turn_end": 0,
            "health": 50,
            "cards": [1, 2, 3, 4, 5],
            "discard_cards": [6, 7],
            "opponents": [
                {
                    "player_id": 1,
                    "health": 40,
                    "card_count": 5,
                    "discard_cards": [6, 7]
                },
                {
                    "player_id": 2,
                    "health": 41,
                    "card_count": 3,
                    "discard_cards": [6, 7]
                },
            ],
            "cards_in_pile": 15
        }));
    }

    // chat

    const $log = document.querySelector('#log')
    const $form = document.querySelector('#chatform')
    const $input = document.querySelector('#text')

    const $conn = document.querySelector('#connection-status')

    function log(msg, type = 'status') {
        $log.innerHTML += `<p class="msg msg--${type}">${msg}</p>`
        $log.scrollTop += 1000
    }

    GAME.serverConnexion.addEventListener("chatmessage", ev => {
        log(ev.detail.message);
    })

    GAME.serverConnexion.addEventListener("connectionchange", ev => {
        $conn.textContent = (ev.detail.status ? "Connected" : "Not connected");
    })

    $form.addEventListener('submit', ev => {
        ev.preventDefault()

        const text = $input.value

        GAME.serverConnexion.send_chat_message(text);

        log(text);

        $input.value = ''
        $input.focus()
    })

    // GAME.connectToServer(wsUrl);
    GAME.onSessionInfoReceived(session_info);
} else {
    window.location.href = "/index.html";
}
