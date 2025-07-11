import { displayPopup } from "../ui/popup";
import { api_url } from "../utils";

export async function request_password_reset(email) {
    const payload = {
        "email": email,
    };

    try {
        const response = await fetch(api_url("/account/request-password-reset"), {
            method: "POST",
            body: JSON.stringify(payload),
            headers: {
                "Content-Type": "application/json"
            },
        });

        if (!response.ok)
            throw new Error(response.statusText);

        displayPopup("An Email was sent to the provided address. Please check your inbox and follow the instructions to recover your account.",
            "Recovery Email Sent", "Back to Login", () => { window.location.href = "/login.html"; });
    } catch (error) {
        console.error("Error: " + error.message);
        displayPopup("An error occurred while requesting the password reset. Please try again later.", "Error");
    }
}


export async function reset_password(token, new_password) {
    const payload = {
        "token": token,
        "new_password": new_password,
    };

    try {
        const response = await fetch(api_url("/account/reset-password"), {
            method: "POST",
            body: JSON.stringify(payload),
            headers: {
                "Content-Type": "application/json"
            },
        });

        if (!response.ok)
            throw new Error(await response.text());

        displayPopup("Your password has been successfully reset. You can now login with your new password.",
            "Password Reset Successful", "Login", () => { window.location.href = "/login.html"; });

    } catch (error) {
        displayPopup("An error occurred while resetting your password: " + error.message, "Error");
    }
}
