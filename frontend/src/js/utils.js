import { get_my_account } from "./api/account";
import { AccountDTO } from "./api/dto";
import { displayMessageNoControls, displayPopup } from "./ui/popup";
import { APP_STATE } from "./app_state";

export const HOST = "localhost:8080";
export const API_URL = `http://${HOST}`;
export const WS_URL = `ws://${HOST}/ws`;


export function api_url(endpoint) { return API_URL + endpoint; }
export function ws_url(game_id) { return `${WS_URL}/${game_id}`; }


export function getCookie(name) {
    const match = document.cookie
                  .match(new RegExp("(^| )" + name + "=([^;]+)"));
    if (match) return match[2];
    return null;
}


/** @type {AccountDTO | null} */
export async function login_guard(silent=true) {
    if (silent) {

        try {
            const account = await get_my_account();
            if (account == null)
                throw new Error("You are not logged in !")

            APP_STATE.account = account;
            return account;
        } catch (error) {
            console.log("Error: " + error.message);
            window.location.href = "/login.html";
            APP_STATE.account = account;
            return null;
        }

    } else {

        const msgFrame = displayMessageNoControls("Connecting...");

        try {
            if (!getCookie("token"))
                throw new Error("You are not logged in !")

            const account = await get_my_account();
            msgFrame.remove();
            APP_STATE.account = account;
            return account;
        } catch (error) {
            msgFrame.remove();

            displayPopup(error.message, "Error", "Go to Login Page", () => {
                window.location.href = "/login.html";
            })
            APP_STATE.account = account;
            return null;
        }

    }
    
}


export function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}


/**
 * @param {Array<string>} strings
 * @param {string} sep
*/
export function strjoin(strings, sep, skip_empty=false) {
    let result = "";
    let strarr = strings;

    if (skip_empty) {
        strarr = strarr.filter(i => Boolean(i));
    }

    for (let i = 0; i < strarr.length; i++) {
        if (!(!strarr[i] && skip_empty)) {
            result += strarr[i];
            if (i+1 < strarr.length) result += sep;
        }
    }
    return result;
}
