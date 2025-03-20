function getCookie(name) {
    const match = document.cookie
                  .match(new RegExp("(^| )" + name + "=([^;]+)"));
    if (match) return match[2];
    return null;
}

if (!getCookie("token")) {
    alert("Not logged in!");
    window.location.href = "/login.html";
}

import * as THREE from 'three';
import * as GAME from './game';

GAME.initGame();
