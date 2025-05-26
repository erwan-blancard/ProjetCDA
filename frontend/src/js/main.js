import gsap from "gsap";


screen.orientation.lock("landscape");


function getCookie(name) {
    const match = document.cookie
                  .match(new RegExp("(^| )" + name + "=([^;]+)"));
    if (match) return match[2];
    return null;
}

// if (!getCookie("token")) {
//     alert("Not logged in!");
//     window.location.href = "/login.html";
// }

import * as THREE from 'three';
import * as GAME from './game';

const server_host = "localhost:8081";
const wsUri = `ws://${server_host}/ws`;

let token = getCookie("token");
if (token == null) {
    token = "";
}

GAME.initGame();

// chat

const $log = document.querySelector('#log')
const $form = document.querySelector('#chatform')
const $input = document.querySelector('#text')

const $conn = document.querySelector('#connection-status')
const $auth = document.querySelector('#auth-status')

function log(msg, type = 'status') {
    $log.innerHTML += `<p class="msg msg--${type}">${msg}</p>`
    $log.scrollTop += 1000
}

GAME.serverConnexion.addEventListener("chatmessage", ev => {
    log(ev.detail.msg);
})

GAME.serverConnexion.addEventListener("connectionchange", ev => {
    $conn.textContent = (ev.detail.status ? "Connected" : "Not connected");
})

GAME.serverConnexion.addEventListener("authchange", ev => {
    $auth.textContent = (ev.detail.status ? "Ok" : "Fail");
})

$form.addEventListener('submit', ev => {
    ev.preventDefault()

    const text = $input.value

    GAME.serverConnexion.send_chat_message(text);

    log(text);

    $input.value = ''
    $input.focus()
})

GAME.connectToServer(wsUri, token);
