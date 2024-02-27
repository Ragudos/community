(function() {
    "use strict";

    window.toaster = {
        init_toast,
        delete_toast,
        toast
    };

    const toasts = new Array();
    const alphanumerical = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const icons = {
        success: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4"><path fill-rule="evenodd" d="M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14Zm3.844-8.791a.75.75 0 0 0-1.188-.918l-3.7 4.79-1.649-1.833a.75.75 0 1 0-1.114 1.004l2.25 2.5a.75.75 0 0 0 1.15-.043l4.25-5.5Z" clip-rule="evenodd" /></svg>`,
        info: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4"><path fill-rule="evenodd" d="M15 8A7 7 0 1 1 1 8a7 7 0 0 1 14 0ZM9 5a1 1 0 1 1-2 0 1 1 0 0 1 2 0ZM6.75 8a.75.75 0 0 0 0 1.5h.75v1.75a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8.25 8h-1.5Z" clip-rule="evenodd" /></svg>`,
        warning: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M6.701 2.25c.577-1 2.02-1 2.598 0l5.196 9a1.5 1.5 0 0 1-1.299 2.25H2.804a1.5 1.5 0 0 1-1.3-2.25l5.197-9ZM8 4a.75.75 0 0 1 .75.75v3a.75.75 0 1 1-1.5 0v-3A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd" /></svg>`,
        error: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14ZM8 4a.75.75 0 0 1 .75.75v3a.75.75 0 0 1-1.5 0v-3A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" clip-rule="evenodd" /></svg>`
    };

    function _get_toast_id() {
        let id = "";
        for (let i = 0; i < 10; i++) {
            id += alphanumerical.charAt(Math.floor(Math.random() * (alphanumerical.length - 1)));
        }
        return id;
    }

    function _create_toast(type, message) {
        const id = _get_toast_id();
        const toast = document.createElement("div");
        toast.id = id;
        toast.className = `toast toast__${type}`;
        
        const icon_container = document.createElement("div");
        icon_container.className = "toast__icon";
        icon_container.innerHTML = icons[type];

        const message_container = document.createElement("div");
        message_container.className = "toast__message";
        message_container.appendChild(document.createTextNode(message));

        const container = document.createElement("div");
        container.className = "toast__container";
        container.appendChild(icon_container);
        container.appendChild(message_container);

        toast.appendChild(container);

        toasts.push({
            toast,
            timeout: setTimeout(() => {
                toast.remove();
                toasts.splice(toasts.findIndex(t => t.toast.id === id), 1);
            }, 5_000)
        });
        return toast;
    }

    function init_toast() {
        if (document.getElementById("__toaster__") != null) {
            console.warn("Toaster already initialized");
            return;
        }

        const toaster = document.createElement("div");
        toaster.id = "__toaster__";
        toaster.ariaLabel = "Notifications";

        document.body.appendChild(toaster);
    }

    function toast(type, message) {
        const toaster = document.getElementById("__toaster__");
        
        let toast = _create_toast(type, message);
        toaster.appendChild(toast);
    }

    function delete_toast(id) {
        const toaster = document.getElementById("__toaster__");
        let idx; 
        
        let toast = toasts.find((t, index) => {
            idx = index;
            return t.toast.id === id
        });

        clearTimeout(toast.timeout);
        toast.toast.remove();
        
        if (idx !== undefined) {
            toasts.splice(idx, 1);
        }
    }
})();
