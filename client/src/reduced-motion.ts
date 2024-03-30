export function initReducedMotion() {
    const mediaQuery = window.matchMedia("(prefers-reduced-motion: reduce)");

    mediaQuery.addEventListener("change", mediaQueryListener);

    if (mediaQuery.matches) {
        document.documentElement.classList.add("reduce-motion");
    } else {
        document.documentElement.classList.remove("reduce-motion");
    }
}

function mediaQueryListener(evt: MediaQueryListEvent) {
    if (evt.matches) {
        document.documentElement.classList.add("reduce-motion");
    } else {
        document.documentElement.classList.remove("reduce-motion");
    }
}
