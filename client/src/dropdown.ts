window.addEventListener("click", (evt) => {
    const dropdowns = document.querySelectorAll(".dropdown");
    const target = evt.target as HTMLElement;

    for (let idx = 0; idx < dropdowns.length; ++idx) {
        const dropdown = dropdowns[idx];
        const btn = dropdown.querySelector("button[data-opener]");

        if (dropdown.contains(target)) {
            continue;
        }

        const isOpen = btn.getAttribute("aria-expanded") === "true";

        if (isOpen) {
            btn.setAttribute("aria-expanded", "false");
            const targetId = btn.getAttribute("aria-controls");
            const target = document.getElementById(targetId);

            if (!target) {
                return console.warn(`No target found for dropdown with id ${targetId}`);
            }

            target.removeAttribute("data-active")
            const btns = [...target.querySelectorAll("a"), ...target.querySelectorAll("button")];

            for (let idx = 0; idx < btns.length; ++idx) {
                btns[idx].removeAttribute("tabindex");
            }
        }
    }
});

document.querySelectorAll(".dropdown").forEach((dropdown) => {
    const btn = dropdown.querySelector("button[data-opener]")

    btn.addEventListener("click", (evt) => {
        const targetId = btn.getAttribute("aria-controls");
        const target = document.getElementById(targetId);

        if (!target) {
            return console.warn(`No target found for dropdown with id ${targetId}`);
        }

        const isOpen = btn.getAttribute("aria-expanded") === "true";

        if (isOpen) {
            btn.setAttribute("aria-expanded", "false");
            target.removeAttribute("data-active")
            const btns = [...target.querySelectorAll("a"), ...target.querySelectorAll("button")];

            for (let idx = 0; idx < btns.length; ++idx) {
                btns[idx].removeAttribute("tabindex");
            }
        } else {
            btn.setAttribute("aria-expanded", "true");
            target.setAttribute("data-active", "true");

            const btns = [...target.querySelectorAll("a"), ...target.querySelectorAll("button")];

            for (let idx = 0; idx < btns.length; ++idx) {
                btns[idx].setAttribute("tabindex", "0");
            }
        }
    });
})