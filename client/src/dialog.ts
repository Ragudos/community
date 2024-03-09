document.addEventListener("DOMContentLoaded", () => {
    document.querySelectorAll("dialog").forEach((dialog) => {
        dialog.addEventListener("click", (evt) => {
            for (const el of dialog.querySelectorAll("*")) { 
                if (el === evt.target) {
                    return;
                }
            }

            dialog.close();
        });
    });
});
