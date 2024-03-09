document.querySelectorAll("[data-checkbox-list]").forEach((el) => {
    let checked = 0;
    const checkboxes = el.querySelectorAll("input[type='checkbox']");
    const maxPossibleChecks = parseInt(el.getAttribute("data-max") || checkboxes.length.toString());
    
    for (let i = 0; i < checkboxes.length; i++) {
        const checkbox = checkboxes[i];

        if (!(checkbox instanceof HTMLInputElement)) {
            continue;
        }

        checkbox.checked = false;

        checkbox.addEventListener("change", (e) => {
            if (checked >= maxPossibleChecks && checkbox.checked) {
                checkbox.checked = false;
                
                return;
            }

            if (checked === maxPossibleChecks && !checkbox.checked) {
                for (let j = 0; j < checkboxes.length; j++) {
                    const checkbox = checkboxes[j];
                    
                    // @ts-ignore
                    if (checkbox.getAttribute("disabled") !== null || checkbox.getAttribute("disabled") !== undefined) {
                        // @ts-ignore
                        checkbox.removeAttribute("disabled");
                    }
                }
            }

            if (checkbox.checked) {
                checked++;
            } else {
                checked--;
            }

            if (checked === maxPossibleChecks) {
                for (let j = 0; j < checkboxes.length; j++) {
                    const checkbox = checkboxes[j];
                    
                    // @ts-ignore
                    if (!checkbox.checked) {
                        // @ts-ignore
                        checkbox.disabled = true;
                    }
                }
            }
        });
    }
});