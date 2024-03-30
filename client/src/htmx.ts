export function initHtmx() {
    const timers = new Array();

    document.addEventListener("htmx:sendAbort", onAbort);
    document.addEventListener("htmx:sendError", onError);

    window.htmx.defineExtension("remove-me", {
        onEvent: removeMeExtension(timers),
    });

    document.addEventListener("HxEvent:Toast", (evt) => {
        console.log(evt);
    });
}

function onError(evt: CustomEvent) {
    return (evt: CustomEvent) => {
        const toastContainer = getToastContainer();
    };
}

function onAbort(evt: CustomEvent) {
    return (evt: CustomEvent) => {
        const toastContainer = getToastContainer();
    };
}

function removeMeExtension(timers: number[]) {
    const maybeRemoveMe = (elt: HTMLElement) => {
        const time =
            elt.getAttribute("remove-me") || elt.getAttribute("data-remove-me");

        if (!time) {
            return;
        }

        const parsedTime = window.htmx.parseInterval(time);
        const timeout = setTimeout(() => {
            const timerIndex = timers.indexOf(timeout);

            if (timerIndex !== -1) {
                timers.splice(timerIndex, 1);
            }

            removeElement(elt);
        }, parsedTime);
    };

    return (name: string, evt: CustomEvent) => {
        if (name != "htmx:afterProcessNode") {
            return;
        }

        const elt = evt.detail.elt;

        if (elt instanceof HTMLElement && elt.getAttribute) {
            maybeRemoveMe(elt);

            if (elt.querySelectorAll) {
                const children = elt.querySelectorAll(
                    "[remove-me], [data-remove-me]",
                );

                for (let i = 0; i < children.length; ++i) {
                    if (children[i] instanceof HTMLElement) {
                        maybeRemoveMe(children[i] as HTMLElement);
                    }
                }
            }
        }
    };
}

function removeElement(element: HTMLElement) {
    element.remove();
}

function getToastContainer(): HTMLElement {
    const toastContainer = document.getElementById("toast-container");

    if (!(toastContainer instanceof HTMLElement)) {
        throw new Error("Toast Container doesn't exist.");
    }

    return toastContainer;
}

function getToastWarningTemplate(): HTMLElement {
    const toastWarning = document.getElementById("toast-warning");

    if (!(toastWarning instanceof HTMLElement)) {
        throw new Error("Toast Warning template doesn't exist.");
    }

    return toastWarning;
}

function getToastErrorTemplate(): HTMLElement {
    const toastError = document.getElementById("toast-error");

    if (!(toastError instanceof HTMLElement)) {
        throw new Error("Toast Error template doesn't exist.");
    }

    return toastError;
}

function getToastSuccessTemplate(): HTMLElement {
    const toastSuccess = document.getElementById("toast-success");

    if (!(toastSuccess instanceof HTMLElement)) {
        throw new Error("Toast Success template doesn't exist.");
    }

    return toastSuccess;
}

function getToastInfoTemplate(): HTMLElement {
    const toastInfo = document.getElementById("toast-info");

    if (!(toastInfo instanceof HTMLElement)) {
        throw new Error("Toast Info template doesn't exist.");
    }

    return toastInfo;
}

function getToastNeutralTemplate(): HTMLElement {
    const toastNeutral = document.getElementById("toast-neutral");

    if (!(toastNeutral instanceof HTMLElement)) {
        throw new Error("Toast Neutral template doesn't exist.");
    }

    return toastNeutral;
}
