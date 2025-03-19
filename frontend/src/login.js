

function login_submit() {
    const username = document.getElementById("username-input").value;
    const password = document.getElementById("password-input").value;

    const payload = {
        "username": username,
        "password": password
    };

    const request = new XMLHttpRequest();

    // when request was sent
    request.onreadystatechange = () => {
        // Call a function when the state changes.
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
    // request.setRequestHeader("Accept", "text/plain");
    request.send(JSON.stringify(payload));

}