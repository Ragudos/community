import { initDialog } from "./dialog";
import { initDropdown } from "./dropdown";
import { initHtmx } from "./htmx";
import { InputWithCounter } from "./web/input_with_counter";

function init() {
    initHtmx();
    initDropdown();
    initDialog();

    customElements.define("input-with-counter", InputWithCounter);
}

window.addEventListener("DOMContentLoaded", init);
