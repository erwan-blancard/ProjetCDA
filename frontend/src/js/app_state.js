import { AccountDTO } from "./api/dto";

/** Store the account (and more) to be used by other modules */
export const APP_STATE = {
    /** @type {AccountDTO | null} */
    account: null
};
