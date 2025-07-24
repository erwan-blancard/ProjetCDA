import { login_guard, ws_url } from "./utils";

import * as THREE from 'three';
import * as GAME from './game/game';
import { get_current_game_info } from "./api/account";
import { CARD_COLLECTION } from "./game/collection";
import { displayMessageNoControls } from "./ui/popup";


// expose for debug
window.GAME = GAME;


screen.orientation.lock("landscape");


const account = await login_guard();

const game_info = await get_current_game_info();


if (game_info != null) {
    const wsUrl = ws_url(game_info.game_id);

    try {
        GAME.initGame();
    } catch (e) {
        displayMessageNoControls(`Error when initializing Game: ${e.message}`);
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

    GAME.connectToServer(wsUrl);
} else {
    window.location.href = "/index.html";
}
