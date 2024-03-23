export function initHtmx() {
    document.addEventListener("htmx:sendAbort", onAbort);
}

function onAbort(evt: CustomEvent) {
    const toastContainer = document.getElementById("toast-container");

    if (!(toastContainer instanceof HTMLElement)) {
        return console.warn("Toast Container doesn't exist.");
    }

    const toaster = document.createElement("li");

    toaster.classList.add("toaster");
    toaster.setAttribute("data-type", "warning");

    const icon = document.createElement("i");

    icon.classList.add("fa-solid", "fa-triangle-exclamation");

    const text = document.createElement("p");

    text.textContent = "You aborted a request. Please refresh the page.";

    toaster.appendChild(icon);
    toaster.appendChild(text);

    toastContainer.appendChild(toaster);

    setTimeout(() => {
        toaster.remove();
    }, 4000);
}
