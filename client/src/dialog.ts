export function initDialog() {
    window.openDialog = function (el: HTMLButtonElement) {
        const targetId = el.getAttribute("aria-controls"),
            target = document.getElementById(targetId);

        if (!(target instanceof HTMLDialogElement)) {
            return console.warn("Target doesn't exist for ", el);
        }

        const onDialogClickFn = onDialogClick(target);

        target.addEventListener("click", onDialogClickFn);
        target.addEventListener(
            "close",
            onDialogClose(target, [onDialogClickFn]),
            { once: true },
        );

        target.showModal();
    };
    window.closeDialog = function(el: HTMLButtonElement) {
        const targetId = el.getAttribute("aria-controls"),
            target = document.getElementById(targetId);

        if (!(target instanceof HTMLDialogElement)) {
            return console.warn("Target doesn't exist for ", el);
        }

        target.close();
    }
}

function onDialogClose(
    dialog: HTMLDialogElement,
    fn: ((...args: any) => void)[],
) {
    return (evt: CloseEvent) => {
        for (let i = 0; i < fn.length; ++i) {
            const f = fn[i];

            dialog.removeEventListener("click", f);
        }
    };
}

function onDialogClick(dialog: HTMLDialogElement) {
    return (evt: MouseEvent) => {
        const dialogChildren = dialog.querySelectorAll("*");

        for (let i = 0; i < dialogChildren.length; ++i) {
            const el = dialogChildren[i];

            if (el === evt.target) {
                return;
            }
        }

        dialog.close();
    };
}
