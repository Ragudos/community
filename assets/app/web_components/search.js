class SearchInput extends HTMLElement {
    constructor() {
        super();
        this._shadowRoot = this.attachShadow({ mode: "open" });
    }

    connectedCallback() {
        const container = document.createElement("label");
        container.setAttribute("class", "search-container");
        
        const input = document.createElement("input");
        input.setAttribute("type", "text");
        input.setAttribute("class", "search-input");
        input.setAttribute("placeholder", "Search...");
        input.setAttribute("hx-post", this.hx_post);
        input.setAttribute("name", this.name);

        if (this.hx_target) {
            input.setAttribute("hx-target", this.hx_target)
        };
        if (this.hx_trigger) {
            input.setAttribute("hx-trigger", this.hx_trigger);
        }

        const icon_container = document.createElement("span");
        icon_container.innerHTML = `<svg style="width: 1.25em; height: 1.25em;"  xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z" clip-rule="evenodd" /></svg>`;

        container.appendChild(icon_container);
        container.appendChild(input);
        this._shadowRoot.appendChild(container);
    }
    
    get hx_target() {
        return this.getAttribute("hx-target");
    }

    get hx_trigger() {
        return this.getAttribute("hx-trigger");
    }

    get hx_post() {
        return this.getAttribute("hx-post");
    }

    get name() {
        return this.getAttribute("name");
    }
}

export { SearchInput }
