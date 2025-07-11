import { api_url } from "../utils";
import { displayPopup } from "../ui/popup";

export async function login_submit() {
    const button = document.getElementById("form-submit");

    const username = document.getElementById("username-input").value;
    const password = document.getElementById("password-input").value;

    button.disabled = true;

    await login(username, password);

    button.disabled = false;
}


export async function register_submit() {
    const button = document.getElementById("form-submit");

    const username = document.getElementById("username-input").value;
    const email = document.getElementById("email-input").value;
    const password = document.getElementById("password-input").value;
    const confirm_password = document.getElementById("confirm-password-input").value;

    if (password !== confirm_password) {
        displayPopup("Passwords do not match", "Error");
        return;
    }

    button.disabled = true;

    await register(username, email, password);

    button.disabled = false;
}


export async function login(username, password) {
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
}


export async function register(username, email, password) {
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