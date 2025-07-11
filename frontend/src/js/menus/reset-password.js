import { reset_password } from "../api/settings";
import { displayPopup } from "../ui/popup";
import { ViewMgr } from "../ui/viewmgr";

const viewMgr = new ViewMgr();

// verify if token in url
const urlParams = new URLSearchParams(window.location.search);
const token = urlParams.get("token");

if (!token) {
    // display error view
    viewMgr.setPrimaryView("view-invalid-token");
} else {
    // display form
    viewMgr.setPrimaryView("view-reset-password");
}

async function reset_password_submit() {
    const password = document.getElementById("password-input").value;
    const confirm_password = document.getElementById("confirm-password-input").value;
    const input_status = document.getElementById("input-status");

    const button = document.getElementById("form-submit");

    if (password !== confirm_password) {
        input_status.textContent = "Passwords do not match";
        return;
    } else {
        input_status.textContent = "";
    }

    button.disabled = true;

    await reset_password(token, password);

    button.disabled = false;
}

// expose function
window.reset_password_submit = reset_password_submit;