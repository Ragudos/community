document.querySelectorAll("a[data-replace]").forEach((anchor) => {
    if (!(anchor instanceof HTMLAnchorElement)) {
        return;
    }

    anchor.addEventListener("click", (evt) => {
        evt.preventDefault();

        window.location.replace(anchor.href);
    });
});