import { initDialog } from "./dialog";
import { initDropdown } from "./dropdown";
import { initHtmx } from "./htmx";
import { initNavigationProgress } from "./navigation-progress";
import { initNotifications } from "./notifications";
import { initReducedMotion } from "./reduced-motion";
import { InputWithCounter } from "./web/input_with_counter";

function init() {
    initReducedMotion();
    initNotifications();
    initHtmx();
    initNavigationProgress();
    initDropdown();
    initDialog();

    customElements.define("input-with-counter", InputWithCounter);
}

window.addEventListener("DOMContentLoaded", init);
