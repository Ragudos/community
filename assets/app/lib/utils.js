export function getTheme() {
    return document.documentElement.getAttribute('data-theme') ||
        (document.documentElement.classList.contains("light") 
            ? "light"
            : document.documentElement.classList.contains("dark")
                ? "dark"
                : "system");
}
