import { LobbyDTO, LobbyInfoDTO, LobbyPageListDTO } from "./dto";
import { api_url } from "../utils";
import { displayPopup } from "../ui/popup";


/** @type {Promise<LobbyDTO | null>} */
export async function create_lobby(unlisted, warn=true) {
    const payload = {
        "unlisted": unlisted,
    };

    try {
        const response = await fetch(api_url("/lobby/create"), {
            method: 'POST',
            body: JSON.stringify(payload),
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.status == 201)    // not created
            throw new Error(response.statusText);

        return new LobbyDTO(await response.json());
    } catch (error) {
        console.log(`Could not create lobby: ${error.message}`);

        if (warn)
            displayPopup(`Could not create lobby: ${error.message}`, "Error");

        return null;
    }
}


/** @type {Promise<LobbyDTO | null>} */
export async function get_lobby_info(lobby_id) {
    try {
        const response = await fetch(api_url(`/lobby/find/${lobby_id}`), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (response.status != 302)    // not Found
            throw new Error("Lobby not found !");

        return new LobbyInfoDTO(await response.json());
    } catch (error) {
        console.log("Could not find lobby:", error.message);
        return null;
    }
}


/** @type {Promise<LobbyPageListDTO | null>} */
export async function list_lobbies(page) {
    try {
        const response = await fetch(api_url(`/lobby/list/${page}`), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error(response.statusText);

        return new LobbyPageListDTO(await response.json());
    } catch (error) {
        console.log(`Could not list lobbies for page ${page}`, error.message);
        return null;
    }
}


/** @type {Promise<LobbyDTO | null>} */
export async function get_current_lobby() {
    try {
        const response = await fetch(api_url("/lobby/current"), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (response.status != 302)    // not Found
            if (response.status == 404)
                throw new Error("No current lobby !");
            else
                throw new Error("Could not get current lobby !");

        return new LobbyDTO(await response.json());
    } catch (error) {
        console.log("Could not get current lobby:", error.message);
        return null;
    }
}

export async function join_lobby(lobby_id, warn=true) {
    const payload = {
        "lobby_id": lobby_id,
    };

    try {
        const response = await fetch(api_url("/lobby/join"), {
            method: 'POST',
            body: JSON.stringify(payload),
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error(response.statusText);

        return new LobbyDTO(await response.json());
    } catch (error) {
        console.log(`Could not join lobby: ${error.message}`);

        if (warn)
            displayPopup(`Could not join lobby: ${error.message}`, "Error");

        return null;
    }
}

export async function leave_current_lobby() {
    try {
        const response = await fetch(api_url("/lobby/current/leave"), {
            method: 'POST',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error(response.statusText);

        return true;
    } catch (error) {
        console.log(`Could not leave lobby: ${error.message}`);
        return false;
    }
}

export async function current_lobby_set_ready_state(ready) {
    const payload = {
        "ready": ready,
    }

    try {
        const response = await fetch(api_url("/lobby/current/ready"), {
            method: 'PATCH',
            body: JSON.stringify(payload),
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error(response.statusText);

        return true;
    } catch (error) {
        console.log(`Could not set lobby ready state: ${error.message}`);
        return false;
    }
}


// expose functions (test)
window.create_lobby = create_lobby;
window.get_current_lobby = get_current_lobby;
window.list_lobbies = list_lobbies;
window.leave_current_lobby = leave_current_lobby;
window.current_lobby_set_ready_state = current_lobby_set_ready_state;
