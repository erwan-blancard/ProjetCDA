import { request_password_reset } from "../api/settings";
import { displayPopup } from "../ui/popup";

async function forgot_password_submit() {
    const email_input = document.getElementById("email-input");
    const submit_button = document.getElementById("form-submit");
    const back_button = document.getElementById("back-button");
    const input_status = document.getElementById("input-status");

    const email = email_input.value;

    if (email && email.includes("@")) {
        input_status.textContent = "";

        email_input.disabled = true;
        submit_button.disabled = true;
        back_button.disabled = true;

        await request_password_reset(email);

        back_button.disabled = false;
        submit_button.disabled = false;
        email_input.disabled = false;
    } else {
        input_status.textContent = "Please enter a valid email address";
    }
}


// expose function
window.forgot_password_submit = forgot_password_submit;
