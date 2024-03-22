export function initDialog() {
    document.querySelectorAll("dialog").forEach((dialog) => {
        dialog.addEventListener("click", (evt) => {
            const dialogChildren = dialog.querySelectorAll("*");

            for (let i = 0; i < dialogChildren.length; ++i) { 
                const el = dialogChildren[i];

                if (el === evt.target) {
                    return;
                }
            }

            dialog.close();
        });
    });
}
