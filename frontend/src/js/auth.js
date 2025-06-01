import { api_url } from "./api";
import { displayPopup } from "./ui/popup";

export async function login_submit() {
    const button = document.getElementById("form-submit");
    button.disabled = true;

    const username = document.getElementById("username-input").value;
    const password = document.getElementById("password-input").value;

    const payload = {
        "username": username,
        "password": password
    };

    try {
        const response = await fetch(api_url("/login"), {
            method: "POST",
            body: JSON.stringify(payload),
            headers: {
                "Content-Type": "application/json"
            },
            credentials: "include"  // this sets the "token" cookie when response is received
        });

        if (!response.ok)
            throw new Error(`Status: ${response.status}, message: ${await response.text()}`);

        // go to index.html
        // cookie with token should have been updated
        window.location.href = "/index.html";
    } catch (error) {
        displayPopup(`An error occured when logging in: ${error.message}`, "Error");
    }

    button.disabled = false;
}


export async function register_submit() {
    const button = document.getElementById("form-submit");
    button.disabled = true;

    const username = document.getElementById("username-input").value;
    const email = document.getElementById("email-input").value;
    const password = document.getElementById("password-input").value;

    const payload = {
        "username": username,
        "email": email,
        "password": password
    };

    try {
        const response = await fetch(api_url("/register"), {
            method: "POST",
            body: JSON.stringify(payload),
            headers: {
                "Content-Type": "application/json"
            }
        });

        if (!response.ok)
            throw new Error(`Status: ${response.status}, message: ${await response.text()}`);

        displayPopup("Account successfully created !", "Account Created", "Go to Login page",
                    () => {
                        window.location.href = "/login.html";
                    });

    } catch (error) {
        displayPopup(`There was an error when creating the account: ${error.message}`, "Error");
    }

    button.disabled = false;
}


export function logout() {
    // clear token and go to login page
    document.cookie = "token=";
    window.location.href = "/login.html";
}


// expose function inline for onclick tag
window.login_submit = login_submit;
window.register_submit = register_submit;
window.logout = logout;