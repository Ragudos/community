import { mount_toaster, toast } from "@webdevaaron/vanilla-toast";
import { getTheme } from "./utils";
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
            toast.error(
                { message: "Wowza! Please slow down." },
                { style: "plain", theme: getTheme() },
            );
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
        toast.error({ message }, { style: "plain", theme: getTheme() });
    }
});

document.addEventListener("htmx:sendError", (evt) => {
    toast.error(
        { message: "Failed to connect to server." },
        { style: "plain", theme: getTheme() },
    );
});

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
