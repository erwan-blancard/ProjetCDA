/** prevent tabbing to elements outside of the popup by adding inert attribute */
export function setInertExceptPopup(popupFrame) {
    Array.from(document.body.children).forEach(child => {
        if (child !== popupFrame) {
            child.setAttribute('inert', '');
            child.setAttribute('aria-hidden', 'true');
        }
    });
}

export function removeInert() {
    Array.from(document.body.children).forEach(child => {
        child.removeAttribute('inert');
        child.removeAttribute('aria-hidden');
    });
}


export function displayPopup(message, title, buttonText="Ok", onclose=null) {
    const frame = document.createElement("div");
    frame.className = "popup-frame";

    const container = document.createElement("section");
    container.className = "popup-container";

    const titleElement = document.createElement("h1");
    titleElement.textContent = title;

    const messageElement = document.createElement("p");
    messageElement.textContent = message;

    const button = document.createElement("button");
    const buttonSpan = document.createElement("span");
    button.className = "styled";
    buttonSpan.textContent = buttonText;
    button.appendChild(buttonSpan);

    button.onclick = () => {
        frame.remove();
        removeInert();

        if (onclose != null) {
            onclose();
        }
    };

    container.appendChild(titleElement);
    container.appendChild(messageElement);
    container.appendChild(button);

    frame.appendChild(container);

    document.body.appendChild(frame);

    setInertExceptPopup(frame);

    return frame;
}

/** Yes/No popup */
export function displayYesNo(message, title, onYesClose=null, onNoClose=null) {
    const frame = document.createElement("div");
    frame.className = "popup-frame";

    const container = document.createElement("section");
    container.className = "popup-container";

    const titleElement = document.createElement("h1");
    titleElement.textContent = title;

    const messageElement = document.createElement("p");
    messageElement.textContent = message;

    const buttonContainer = document.createElement("div");
    buttonContainer.classList = "hlayout center";

    const acceptButton = document.createElement("button");
    const acceptButtonSpan = document.createElement("span");
    acceptButton.className = "styled";
    acceptButtonSpan.textContent = "Yes";
    acceptButton.appendChild(acceptButtonSpan);

    acceptButton.onclick = () => {
        frame.remove();
        removeInert();

        if (onYesClose != null) {
            onYesClose();
        }
    };

    const cancelButton = document.createElement("button");
    const cancelButtonSpan = document.createElement("span");
    cancelButton.className = "styled";
    cancelButtonSpan.textContent = "No";
    cancelButton.appendChild(cancelButtonSpan);

    cancelButton.onclick = () => {
        frame.remove();
        removeInert();

        if (onNoClose != null) {
            onNoClose();
        }
    };

    container.appendChild(titleElement);
    container.appendChild(messageElement);
    buttonContainer.appendChild(acceptButton);
    buttonContainer.appendChild(cancelButton);
    container.appendChild(buttonContainer);

    frame.appendChild(container);

    document.body.appendChild(frame);

    setInertExceptPopup(frame);

    return frame;
}


/** Simpler popup with only a message and no button */
export function displayMessageNoControls(message) {
    const frame = document.createElement("div");
    frame.className = "msg-frame";

    const container = document.createElement("section");
    container.className = "msg-container";

    const messageElement = document.createElement("p");
    messageElement.textContent = message;

    container.appendChild(messageElement);

    frame.appendChild(container);

    document.body.appendChild(frame);

    return frame;
}


export async function displayInput(label, title, buttonText="Ok", attrs=null, styleAttrs=null, required=true) {
    return new Promise(resolve => {
        const frame = document.createElement("div");
        frame.className = "input-frame";

        const container = document.createElement("section");
        container.className = "input-container";

        const titleElement = document.createElement("h1");
        titleElement.textContent = title;

        const inputForm = document.createElement("div");
        inputForm.className = "input-form";

        const labelElement = document.createElement("p");
        labelElement.textContent = label;

        const inputElement = document.createElement("input");
        
        if (attrs)
            for (var k in attrs) {
                inputElement[k] = attrs[k];
            }

        if (styleAttrs)
            for (var k in styleAttrs) {
                inputElement.style[k] = styleAttrs[k];
            }

        inputForm.appendChild(labelElement);
        inputForm.appendChild(inputElement);

        const buttonContainer = document.createElement("div");
        buttonContainer.classList = "hlayout center";

        const acceptButton = document.createElement("button");
        const acceptButtonSpan = document.createElement("span");
        acceptButton.className = "styled";
        acceptButtonSpan.textContent = buttonText;
        acceptButton.appendChild(acceptButtonSpan);

        acceptButton.onclick = async () => {
            frame.remove();
            removeInert();

            resolve(inputElement.value);
        };

        const cancelButton = document.createElement("button");
        const cancelButtonSpan = document.createElement("span");
        cancelButton.className = "styled";
        cancelButtonSpan.textContent = "Cancel";
        cancelButton.appendChild(cancelButtonSpan);

        cancelButton.onclick = async () => {
            frame.remove();
            removeInert();

            resolve(null);
        };

        if (required) {
            acceptButton.disabled = true;
            // disable accept button if empty
            inputElement.onkeyup = () => {
                acceptButton.disabled = !inputElement.value.length;
            }
        }

        container.appendChild(titleElement);
        container.appendChild(inputForm);
        container.appendChild(buttonContainer);
        buttonContainer.appendChild(acceptButton);
        buttonContainer.appendChild(cancelButton);

        frame.appendChild(container);

        document.body.appendChild(frame);

        setInertExceptPopup(frame);
    });
}
