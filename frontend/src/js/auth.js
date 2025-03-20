

function login_submit() {
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
            if (request.status === 200) {
                // console.log(request.getAllResponseHeaders());
                // console.log(document.cookie);
                window.location.href = "/index.html";
            }
        }
    };

    request.open("POST", "http://localhost:8080/login");
    request.setRequestHeader("Content-Type", "application/json");
    request.withCredentials = true;
    request.send(JSON.stringify(payload));

}


function register_submit() {
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
            if (request.status === 200) {
                window.location.href = "/login.html";
            }
        }
    };

    request.open("POST", "http://localhost:8080/register");
    request.setRequestHeader("Content-Type", "application/json");
    request.send(JSON.stringify(payload));

}


function logout() {
    // clear token and go to login page
    document.cookie = "token=";
    window.location.href = "/login.html";
}