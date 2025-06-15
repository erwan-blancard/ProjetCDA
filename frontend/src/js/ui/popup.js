
export function displayPopup(message, title, buttonText="Ok", onclose=null) {
    const frame = document.createElement("div");
    frame.className = "popup-frame";

    const container = document.createElement("section");
    container.className = "popup-container";

    const titleElement = document.createElement("h1");
    titleElement.textContent = title;

    const messageElement= document.createElement("p");
    messageElement.textContent = message;

    const button = document.createElement("button");
    const buttonSpan = document.createElement("span");
    button.className = "styled";
    buttonSpan.textContent = buttonText;
    button.appendChild(buttonSpan);

    button.onclick = () => {
        frame.remove();

        if (onclose != null) {
            onclose();
        }
    };

    container.appendChild(titleElement);
    container.appendChild(messageElement);
    container.appendChild(button);

    frame.appendChild(container);

    document.body.appendChild(frame);

    return frame;
}


// Simpler popup with only a message and no button
export function displayMessage(message) {
    const frame = document.createElement("div");
    frame.className = "msg-frame";

    const container = document.createElement("section");
    container.className = "msg-container";

    const messageElement= document.createElement("p");
    messageElement.textContent = message;

    container.appendChild(messageElement);

    frame.appendChild(container);

    document.body.appendChild(frame);

    return frame;
}
