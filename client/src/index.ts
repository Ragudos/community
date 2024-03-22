import { initDialog } from "./dialog";
import { initDropdown } from "./dropdown";

function init() {
    initDropdown();
    initDialog();
}

window.addEventListener("DOMContentLoaded", init);
