import { mount_toaster, toast } from "@webdevaaron/vanilla-toast";
mount_toaster();

const timers = [];
let errors = 0;
let timeout: ReturnType<typeof setTimeout>;
let timeout2: ReturnType<typeof setTimeout>;

document.addEventListener("htmx:error", (evt) => {
    if (evt instanceof CustomEvent) {
        clearTimeout(timeout);
        errors++;
        
        if (errors > 10) {
            clearTimeout(timeout2);
            toast.error({ message: "Wowza! Please slow down." },  { style: "plain", theme: document.documentElement.dataset.theme as "light" | "dark" || "light" })
            // @ts-ignore
            evt.target.querySelectorAll("*").forEach((el) => {
                el.setAttribute("disabled", "true");
            });

            timeout2 = setTimeout(() => {
                //@ts-ignore
                evt.target.querySelectorAll("*").forEach((el) => {
                    el.removeAttribute("disabled");
                    errors = 0;
                });
            }, 10_000);
            return;
        }

        timeout = setTimeout(() => {
            errors = 0;
        }, 10_000);
    }
});

document.addEventListener("htmx:responseError", (evt) => {
    if (evt instanceof CustomEvent) {
        const message = evt.detail.xhr?.responseText || "Something went wrong.";
        toast.error({ message }, { style: "plain", theme: document.documentElement.dataset.theme as "light" | "dark" || "light"})
    }
});

document.addEventListener("htmx:sendError", (evt) => {
    toast.error({ message: "Failed to connect to server." }, { style: "plain", theme: document.documentElement.dataset.theme as "light" | "dark" || "light" });
});
