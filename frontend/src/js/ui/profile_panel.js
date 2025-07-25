import { AccountDTO, AccountStatsDTO } from "../api/dto";
import { get_account_stats } from "../api/stats";


const PROFILE_TAB_TITLE = "Profile";
const SETTINGS_TAB_TITLE = "Settings";

const WAITING_FOR_STATS_HTML = "Waiting for stats...";
const ERROR_STATS_HTML = '<span style="color: red;">Error querying stats for user</span>';


export class ProfileStats extends HTMLElement {
    /** @type {AccountStatsDTO | null} */
    accountStatsDTO;

    constructor(accountStatsDTO) {
        super();

        this.innerHTML = WAITING_FOR_STATS_HTML;

        this.update(accountStatsDTO);
    }

    update(accountStatsDTO) {
        if (accountStatsDTO) {
            this.accountStatsDTO = accountStatsDTO;

            this.innerHTML = `
                <h3>Stats</h3>
                <p>Level: ${this.accountStatsDTO.level+1}</p>
                <p>Games played: ${this.accountStatsDTO.games_played}</p>
                <p>Games won: ${this.accountStatsDTO.games_won}</p>
            `;
        }
    }

}


/**
 * Element to show profile info
 */
export class ProfileInfo extends HTMLElement {
    /** @type {AccountDTO | null} */
    accountDTO;

    username;
    profileStats;

    constructor(accountDTO) {
        super();

        this.username = document.createElement("h3");
        this.username.textContent = "...";
        this.profileStats = new ProfileStats();

        this.innerHTML += '<div class="hline" style="width: 25%"></div>';
        // if appendChild() was used before innerHTML += ..., ref to this.username
        // would have been lost because new elements would have been created
        this.prepend(this.username);
        this.appendChild(this.profileStats);

        this.update(accountDTO);
    }
    
    async update(accountDTO) {
        if (accountDTO) {
            this.accountDTO = accountDTO;

            this.username.textContent = this.accountDTO.username;

            const accountStatsDTO = await get_account_stats(this.accountDTO.id);
            if (accountStatsDTO) {
                // TODO show stats
                this.profileStats.update(accountStatsDTO);
            } else {
                this.profileStats.innerHTML = ERROR_STATS_HTML;
            }
        }
    }
}


/** 
 * Panel to show our profile
 */
export class ProfilePanel extends HTMLElement {
    /** @type {AccountDTO | null} */
    accountDTO;

    closeBtn;
    profileInfo;
    settings;

    constructor(accountDTO) {
        super();

        this.className = "floating-panel";

        this.closeBtn = document.createElement("i");
        this.closeBtn.className = "fas fa-times-circle";
        this.closeBtn.style.position = "absolute";
        this.closeBtn.style.top = "0.4em";
        this.closeBtn.style.right = "0.4em";
        this.closeBtn.style.cursor = "pointer";

        const tabContainer = document.createElement("div");
        tabContainer.className = "tab-container";

        // tab buttons
        const profileTab = document.createElement("button");
        profileTab.className = "tab";
        profileTab.textContent = PROFILE_TAB_TITLE;
        profileTab.onclick = () => this.switchTab("profile-tab");
        const settingsTab = document.createElement("button");
        settingsTab.className = "tab";
        settingsTab.textContent = SETTINGS_TAB_TITLE;
        settingsTab.onclick = () => this.switchTab("settings-tab");

        tabContainer.appendChild(profileTab);
        tabContainer.appendChild(settingsTab);

        // tab contents
        this.profileInfo = new ProfileInfo(accountDTO);
        this.profileInfo.id = "profile-tab";
        this.profileInfo.className = "tab-content";
        this.settings = document.createElement("div");
        this.settings.id = "settings-tab";
        this.settings.className = "tab-content";

        this.appendChild(this.closeBtn);
        this.appendChild(tabContainer);

        this.appendChild(this.profileInfo);
        this.appendChild(this.settings);

        this.switchTab("profile-tab");

        this.update(accountDTO);
    }

    async update(accountDTO) {
        if (accountDTO) {
            this.accountDTO = accountDTO;

            this.profileInfo.update(accountDTO);

        }

    }

    switchTab(tabId) {
        let contents = Array.from(this.getElementsByClassName("tab-content"));

        contents.forEach(c => {
          c.style.display = c.id == tabId ? "block" : "none";
        });
    }

}


/** 
 * Panel to show any profile
 */
export class OtherProfilePanel extends HTMLElement {
    /** @type {AccountDTO | null} */
    accountDTO;

    closeBtn;
    profileInfo;

    constructor(accountDTO) {
        super();

        this.className = "floating-panel";

        this.closeBtn = document.createElement("i");
        this.closeBtn.className = "fas fa-times-circle";
        this.closeBtn.style.position = "absolute";
        this.closeBtn.style.top = "0.4em";
        this.closeBtn.style.right = "0.4em";
        this.closeBtn.style.cursor = "pointer";

        const title = document.createElement("h3");
        title.className = "panel-title";
        title.textContent = "Profile";

        this.profileInfo = new ProfileInfo(accountDTO);

        this.appendChild(this.closeBtn);
        this.appendChild(title);
        this.appendChild(this.profileInfo);

        this.update(accountDTO);
    }

    async update(accountDTO) {
        if (accountDTO) {
            this.accountDTO = accountDTO;

            this.profileInfo.update(accountDTO);
        }

    }

}


customElements.define("profile-stats", ProfileStats);
customElements.define("profile-info", ProfileInfo);
customElements.define("profile-panel", ProfilePanel);
customElements.define("other-profile-panel", OtherProfilePanel);
