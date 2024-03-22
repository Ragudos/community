export function initDropdown() {
    document.addEventListener("click", onDocumentClick);
    activateDropdowns();
}

function onDocumentClick(evt: MouseEvent) {
    const openDropdown = document.querySelector(".dropdown[data-active=open]"),
        documentTarget = evt.target;

    if (!(documentTarget instanceof HTMLElement)) {
        return;
    }

    const button = openDropdown.querySelector("button[data-opener]") as HTMLButtonElement | null;

    if (!button) {
        return console.warn("No button found in dropdown");
    }

    const targetId = button.getAttribute("aria-controls"),
        target = document.getElementById(targetId);

    if (!target) {
        return console.warn(`No target found for dropdown with id ${targetId}`);
    }

    openDropdown.removeAttribute("data-active");
    button.setAttribute("aria-expanded", "false");
    removeTabIndexFromButtons(getAllButtons(target));
}

function activateDropdowns() {
    const dropdowns = document.querySelectorAll(".dropdown");

    for (let i = 0; i < dropdowns.length; ++i) {
        const dropdown = dropdowns[i],
            button = dropdown.querySelector("button[data-opener]") as HTMLButtonElement | null;

        if (!button) {
            console.warn("No button found in dropdown ", dropdown);
            continue;
        }

        button.addEventListener("click", onDropdownButtonClick(dropdown, button));
    }
}

function onDropdownButtonClick(dropdown: Element, button: HTMLButtonElement) {
    return (evt: MouseEvent) => {
        const targetId = button.getAttribute("aria-controls"),
            target = document.getElementById(targetId);

        if (!target) {
            return console.warn(`No target found for dropdown with id ${targetId}`);
        }

        if (button.getAttribute("aria-expanded") === "true") {
            button.setAttribute("aria-expanded", "false");
            dropdown.removeAttribute("data-active");
            removeTabIndexFromButtons(getAllButtons(target));
        } else {
            button.setAttribute("aria-expanded", "true");
            dropdown.setAttribute("data-active", "true");
            addTabIndexToButtons(getAllButtons(target));
        }
    }
}

function removeTabIndexFromButtons(buttons: (HTMLButtonElement | HTMLAnchorElement)[]) {
    for (let idx = 0; idx < buttons.length; ++idx) {
        buttons[idx].removeAttribute("tabindex");
    }
}

function addTabIndexToButtons(buttons: (HTMLButtonElement | HTMLAnchorElement)[]) {
    for (let idx = 0; idx < buttons.length; ++idx) {
        buttons[idx].setAttribute("tabindex", "0");
    }
}

function getAllButtons(el: HTMLElement) {
    return [
        ...el.querySelectorAll("button"),
        ...el.querySelectorAll("a")
    ]
}

