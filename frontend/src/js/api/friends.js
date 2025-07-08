import { api_url } from "../utils";
import { FriendDTO, FriendWithLobbyStatusDTO } from "./dto";


/**
 * @returns {Promise<Array<FriendWithLobbyStatusDTO> | null>}
 */
export async function get_friends() {
    try {
        const response = await fetch(api_url("/account/friends"), {
            method: 'GET',
            credentials: "include",
            headers: { "Content-Type": "application/json" },
        });
        if (!response.ok) throw new Error(await response.text());
        return (await response.json()).map(f => { return new FriendWithLobbyStatusDTO(f); });
    } catch (error) {
        console.error("Could not get friends list:", error.message);
        return null;
    }
}

/**
 * @returns {Promise<Array<FriendDTO> | null>}
 */
export async function get_friend_requests() {
    try {
        const response = await fetch(api_url("/account/requests"), {
            method: 'GET',
            credentials: "include",
            headers: { "Content-Type": "application/json" },
        });
        if (!response.ok) throw new Error(await response.text());
        return (await response.json()).map(f => { return new FriendDTO(f); });
    } catch (error) {
        console.error("Could not get friend requests:", error.message);
        return null;
    }
}

/**
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
        console.error("Could not send friend request:", error.message);
        return null;
    }
}

/**
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
        return new FriendDTO(await response.json());
    } catch (error) {
        console.error("Could not respond to friend request:", error.message);
        return null;
    }
}

/**
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
        console.error('Could not delete friend:', error.message);
        return null;
    }
}
