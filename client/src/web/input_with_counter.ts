export class InputWithCounter extends HTMLElement {
    public input: HTMLInputElement | HTMLTextAreaElement | null;
    public listeners: (() => void)[];

    constructor() {
        super();
        const shadow = this.attachShadow({ mode: "open" });
        const template = document.getElementById("input-with-counter");

        if (!template || !(template instanceof HTMLTemplateElement)) {
            throw new Error("InputWithCounter: Template not found.");
        }

        shadow.append(template.content.cloneNode(true));
        this.shadowRoot.querySelector(".max-count")!.textContent =
            this.maxCount.toString();
        this.listeners = [];
    }

    connectedCallback() {
        const slots = [...this.shadowRoot.querySelectorAll("slot")];

        for (let i = 0; i < slots.length; ++i) {
            this.listeners.push(this.listenToSlotChange(slots[i]));
        }
    }

    disconnectedCallback() {
        for (let i = this.listeners.length - 1; i >= 0; --i) {
            this.listeners[i]();
            this.listeners.pop();
        }
    }

    listenToSlotChange(slot: HTMLSlotElement) {
        const name = "slotchange";
        const fn = (evt: Event) => {
            const nodes = slot.assignedNodes();
            const name = slot.getAttribute("name");

            if (name == "input" && nodes.length != 1) {
                throw new Error(
                    "InputWithCounter: There must only be one input element in the input slot.",
                );
            }

            if (
                name == "input" &&
                !(nodes[0] instanceof HTMLInputElement) &&
                !(nodes[0] instanceof HTMLTextAreaElement)
            ) {
                throw new Error(
                    "InputWithCounter: The input slot must contain an input or textarea element.",
                );
            }

            this.input = nodes[0] as HTMLInputElement | HTMLTextAreaElement;

            const eventName = "input";
            const inputFn = (kbEvt: KeyboardEvent) => {
                if (this.input.value.length > this.maxCount) {
                    this.input.value = this.input.value.slice(0, this.maxCount);
                    return kbEvt.preventDefault();
                }

                const counter = this.shadowRoot.querySelector(".counter");

                if (!counter) {
                    throw new Error(
                        "InputWithCounter: Counter element not found.",
                    );
                }

                counter.textContent = this.input.value.length.toString();
            };

            this.input.addEventListener(eventName, inputFn);
            this.listeners.push(() => {
                this.input.removeEventListener(eventName, inputFn);
            });
        };

        slot.addEventListener(name, fn);

        return () => {
            slot.removeEventListener(name, fn);
        };
    }

    get maxCount() {
        const maxCount = this.getAttribute("data-max-count");

        if (!maxCount || isNaN(parseInt(maxCount))) {
            console.error(
                "InputWithCounter: Invalid [data-max-count] attribute value.",
            );

            return 0;
        }

        return parseInt(maxCount);
    }

    static get observedAttributes() {
        return ["value"];
    }

    attrbuteChangedCallback(name: string, oldValue: any, newValue: any) {
        console.log(name, oldValue, newValue);
    }
}
