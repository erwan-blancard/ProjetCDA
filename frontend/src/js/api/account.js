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
                throw new Error(`Unexpected response code ${response.status}`);

        return new GameSessionInfoDTO(await response.json());
    } catch (error) {
        console.log("Could not get current game info:", error.message);
        return null;
    }
    
}

/**
 * Récupère la liste des amis de l'utilisateur connecté
 * @returns {Promise<Array| null>}
 */
export async function get_friends() {
    try {
        const response = await fetch(api_url("/account/friends"), {
            method: 'GET',
            credentials: "include",
            headers: { "Content-Type": "application/json" },
        });
        if (!response.ok) throw new Error("Impossible de récupérer la liste d'amis");
        return await response.json();
    } catch (error) {
        console.error("Erreur get_friends:", error.message);
        return null;
    }
}

/**
 * Récupère la liste des demandes d'amis reçues/en attente
 * @returns {Promise<Array| null>}
 */
export async function get_friend_requests() {
    try {
        const response = await fetch(api_url("/account/requests"), {
            method: 'GET',
            credentials: "include",
            headers: { "Content-Type": "application/json" },
        });
        if (!response.ok) throw new Error("Impossible de récupérer les demandes d'amis");
        return await response.json();
    } catch (error) {
        console.error("Erreur get_friend_requests:", error.message);
        return null;
    }
}

/**
 * Envoie une demande d'ami à un utilisateur
 * @param {string} username
 * @returns {Promise<Object|null>}
 */
export async function send_friend_request(username) {
    try {
        const response = await fetch(api_url("/account/requests"), {
            method: 'POST',
            credentials: "include",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username })
        });
        if (!response.ok) throw new Error(await response.text());
        return await response.json();
    } catch (error) {
        console.error("Erreur send_friend_request:", error.message);
        return null;
    }
}

/**
 * Accepte ou refuse une demande d'ami
 * @param {string} username
 * @param {boolean} accepted
 * @returns {Promise<Object|null>}
 */
export async function respond_friend_request(username, accepted) {
    try {
        const response = await fetch(api_url(`/account/requests/${username}`), {
            method: 'PATCH',
            credentials: "include",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ accepted })
        });
        if (!response.ok) throw new Error(await response.text());
        return await response.json();
    } catch (error) {
        console.error("Erreur respond_friend_request:", error.message);
        return null;
    }
}

/**
 * Supprime un ami de la liste
 * @param {string} username
 * @returns {Promise<Object|null>}
 */
export async function delete_friend(username) {
    try {
        const response = await fetch(api_url(`/account/friends/${username}`), {
            method: 'DELETE',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
        });
        if (!response.ok) throw new Error(await response.text());
        return await response.json();
    } catch (error) {
        console.error('Erreur delete_friend:', error.message);
        return null;
    }
}
