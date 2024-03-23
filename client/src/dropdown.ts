export function initDropdown() {
    document.addEventListener("click", onDocumentClick);
}

function onDocumentClick(evt: MouseEvent) {
    const openDropdown = document.querySelector("details.dropdown[open]");

    if (!(openDropdown instanceof HTMLDetailsElement)) {
        return;
    }

    const summary = openDropdown.querySelector("summary");

    if (evt.target === summary) {
        return;
    } else {
        const summaryChildren = Array.from(
            summary?.querySelectorAll("*") ?? [],
        );

        for (let i = 0; i < summaryChildren.length; ++i) {
            if (evt.target === summaryChildren[i]) {
                return;
            }
        }
    }

    openDropdown.removeAttribute("open");
    removeTabIndexFromButtons(getAllButtons(openDropdown));
}

function removeTabIndexFromButtons(
    buttons: (HTMLButtonElement | HTMLAnchorElement)[],
) {
    for (let idx = 0; idx < buttons.length; ++idx) {
        buttons[idx].removeAttribute("tabindex");
    }
}

function getAllButtons(el: HTMLElement) {
    return [...el.querySelectorAll("button"), ...el.querySelectorAll("a")];
}
