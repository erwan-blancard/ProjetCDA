import { get_my_account } from "./api/account";
import { AccountDTO } from "./api/dto";
import { displayMessage, displayPopup } from "./ui/popup";

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

            return account;
        } catch (error) {
            console.log("Error: " + error.message);
            window.location.href = "/login.html";
            return null;
        }

    } else {

        const msgFrame = displayMessage("Connecting...");

        try {
            if (!getCookie("token"))
                throw new Error("You are not logged in !")

            const account = await get_my_account();
            msgFrame.remove();
            return account;
        } catch (error) {
            msgFrame.remove();

            displayPopup(error.message, "Error", "Go to Login Page", () => {
                window.location.href = "/login.html";
            })
            return null;
        }

    }
    
}
