import { AccountDTO, GameSessionInfoDTO } from "./dto";
import { api_url } from "../utils";
import { getAccountFromStore, storeAccount } from "../store";


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

        const account = new AccountDTO(await response.json());
        // put account data in cache store
        try {
            await storeAccount(account);
        } catch (error) {
            console.error("Failed to store account in DB: " + error.message);
        }
        return account;
    } catch (error) {
        console.log("Error getting account: " + error.message);
        return null;
    }
}


/** @type {Promise<AccountDTO | null>} */
export async function get_account(account_id) {

    const cachedAccount = await getAccountFromStore(account_id);
    console.log("Account from Store: ", cachedAccount);

    if (cachedAccount != null) return cachedAccount;

    try {
        const response = await fetch(api_url(`/account/profile/${account_id}`), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error("Could not get account !");

        const account = new AccountDTO(await response.json());
        // put account data in cache store
        try {
            await storeAccount(account);
        } catch (error) {
            console.error("Failed to store account in DB: " + error.message);
        }
        return account;
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
