var recaptchaOnLoad = () => {
    document.querySelectorAll(".g-recaptcha").forEach((el) => {
        grecaptcha.render(el, {
            sitekey:"6Lc2hH8pAAAAAC0YCMf8LsPa0O662Dw-iR-wX615",
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
