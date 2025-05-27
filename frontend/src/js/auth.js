import { api_url } from "./api";
import { displayPopup } from "./ui/popup";

export function login_submit() {
    const button = document.getElementById("form-submit");
    button.disabled = true;

    const username = document.getElementById("username-input").value;
    const password = document.getElementById("password-input").value;

    const payload = {
        "username": username,
        "password": password
    };

    const request = new XMLHttpRequest();

    request.onreadystatechange = () => {
        // when response received
        if (request.readyState === XMLHttpRequest.DONE /* && request.status === 200 */) {
            console.log(`${request.status}: ${request.responseText}`);

            // go to index.html
            // cookie with token  should have been updated
            if (request.status === 200) {
                // console.log(request.getAllResponseHeaders());
                // console.log(document.cookie);
                window.location.href = "/index.html";
            } else {
                displayPopup(`An error occured when logging in !\n${request.responseText}`, "Error");
            }

            button.disabled = false;

        }
    };

    request.open("POST", api_url("/login"));
    request.setRequestHeader("Content-Type", "application/json");
    request.withCredentials = true;
    request.send(JSON.stringify(payload));
}


export function register_submit() {
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

    const request = new XMLHttpRequest();

    request.onreadystatechange = () => {
        // when response received
        if (request.readyState === XMLHttpRequest.DONE /* && request.status === 200 */) {
            console.log(`${request.status}: ${request.responseText}`);

            // go to index.html
            if (request.status === 201) {
                displayPopup("Account successfully created !", "Account Created", "Go to Login page",
                    () => {
                        window.location.href = "/login.html";
                    });
            } else {
                displayPopup("There was an error when creating the account !\n" + `${request.status}: ${request.responseText}`, "Error");
            }

            button.disabled = false;
        }
    };

    request.open("POST", api_url("/register"));
    request.setRequestHeader("Content-Type", "application/json");
    request.send(JSON.stringify(payload));

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