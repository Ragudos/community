var recaptchaOnLoad = () => {
    document.querySelectorAll(".g-recaptcha").forEach((el) => {
        grecaptcha.render(el, {
            sitekey: document.querySelector("meta[name=\"g-recaptcha-sitekey\"]").getAttribute("content"),
            theme: document.documentElement.getAttribute("data-theme") === "dark" ? "dark" : "light",
            size: el.dataset.size,
        });
    });
}

document.addEventListener("DOMContentLoaded", () => {
    document.querySelectorAll("form[data-withcaptcha=true]").forEach((el) => {
        el.addEventListener("htmx:responseError", (event) => {
            console.log(event);
        });
    })
});
