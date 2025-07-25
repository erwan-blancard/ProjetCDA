import { getAccountStatsFromStore, storeAccountStats } from "../store";
import { api_url } from "../utils";
import { AccountStatsDTO } from "./dto";


export async function get_my_account_stats() {
    try {
        const response = await fetch(api_url("/account/stats"), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error("Could not get account !");

        return new AccountStatsDTO(await response.json());
    } catch (error) {
        console.log("Error getting account stats: " + error.message);
        return null;
    }
}


export async function get_account_stats(account_id) {
    const cachedStats = await getAccountStatsFromStore(account_id);
    console.log("Account Stats from Store: ", cachedStats);

    if (cachedStats != null) return cachedStats;
    
    try {
        const response = await fetch(api_url(`/account/stats/${account_id}`), {
            method: 'GET',
            credentials: "include",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok)
            throw new Error("Could not get account stats !");

        const accountStats = new AccountStatsDTO(await response.json());
        // put data in cache store
        try {
            await storeAccountStats(accountStats);
        } catch (error) {
            console.error("Failed to store account stats in DB: " + error.message);
        }
        return accountStats;
    } catch (error) {
        console.log("Error getting account stats: " + error.message);
        return null;
    }
}
