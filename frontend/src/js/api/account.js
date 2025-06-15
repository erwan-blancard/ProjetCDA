import { AccountDTO, GameSessionInfoDTO } from "./dto";
import { api_url } from "../utils";


/** @type {Promise<AccountDTO | null>} */
export async function get_my_account() {
    try {
        const response = await fetch(api_url("/account/profile"), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error("Could not get account !");

        return new AccountDTO(await response.json());
    } catch (error) {
        console.log("Error getting account: " + error.message);
        return null;
    }
}


/** @type {Promise<GameSessionInfoDTO | null>} */
export async function get_current_game_info() {
    try {
        const response = await fetch(api_url("/game/current"), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (response.status != 302)    // Found
            if (response.status == 404)
                throw new Error("No current session !");
            else
                throw new Error("Could not get current session info !");

        return new GameSessionInfoDTO(await response.json());
    } catch (error) {
        console.log("Could not get current game info:", error.message);
        return null;
    }
    
}
