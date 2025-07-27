import { api_url } from "../utils";


export async function get_cards_json() {
    try {
        // route is public
        const response = await fetch(api_url("/cards"), {
            method: "GET",
            headers: {
                "Content-Type": "application/json"
            }
        });

        if (!response.ok)
            throw new Error(await response.text());

        return await response.json();
    } catch (error) {
        console.log(`Error when getting cards: ${error.message}`);
        return null;
    }
}