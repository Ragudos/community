(function() {
    "use strict";

    window.toaster = {
        init_toast,
        create_toast,
        delete_toast
    };

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

    function create_toast() {
        const toaster = document.getElementById("__toaster__");
    }

    function delete_toast() {
        const toaster = document.getElementById("__toaster__");
    }
})();
