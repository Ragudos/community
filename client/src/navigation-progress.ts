type Options = KeyframeAnimationOptions;

export function initNavigationProgress() {
    const doesPreferReducedMotion =
        document.documentElement.classList.contains("reduce-motion");
    const options: Options = doesPreferReducedMotion
        ? { duration: 0, fill: "forwards" }
        : { duration: 250, fill: "forwards" };

    document.addEventListener("htmx:xhr:loadstart", onLoadStart(options));
    document.addEventListener("htmx:xhr:progress", onProgress(options));
    document.addEventListener("htmx:xhr:loadend", onLoadEnd());
}

function onLoadStart(options: Options) {
    return (evt: CustomEvent) => {
        const target = evt.target;

        if (!(target instanceof HTMLAnchorElement)) {
            return;
        }

        if (target.classList.contains("active")) {
            return;
        }

        const progressBar = getProgressBar();

        progressBar.animate(
            {
                opacity: 1,
            },
            options,
        );
    };
}

function onProgress(options: Options) {
    return (evt: CustomEvent) => {
        const target = evt.target;

        if (!(target instanceof HTMLAnchorElement)) {
            return;
        }

        if (target.classList.contains("active")) {
            return;
        }

        const progresBar = getProgressBar();
        const progress =
            evt.detail.toast === 0 ? 0 : evt.detail.loaded / evt.detail.total;

        progresBar.animate(
            {
                width: `${progress * 100}%`,
            },
            options,
        );
    };
}

function onLoadEnd() {
    return (evt: CustomEvent) => {
        const target = evt.target;

        if (!(target instanceof HTMLAnchorElement)) {
            return;
        }

        const progressBar = getProgressBar();

        setTimeout(() => {
            progressBar.animate(
                {
                    opacity: 0,
                    width: "0%",
                },
                { duration: 0, fill: "forwards" },
            );
        }, 250);
    };
}

function getProgressBar(): HTMLElement {
    const progressBar = document.getElementById("page-progress-bar");

    if (progressBar instanceof HTMLElement) {
        return progressBar;
    } else {
        throw new Error("Progress bar doesn't exist.");
    }
}
