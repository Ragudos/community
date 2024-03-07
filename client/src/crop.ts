import { toast } from "@webdevaaron/vanilla-toast";
import Cropper from "cropperjs";
import { getTheme } from "./utils";
// Also implement drop listener

const elts = document.querySelectorAll("[data-attach-img]");
const mimeTypesAllowed = ["image/jpeg", "image/png", "image/webp"];

elts.forEach((elt: HTMLInputElement) => {
    elt?.addEventListener("change", (evt) => {
        elt.click()
        if (elt.files.length === 0) {
            return;
        }

        if (elt.files.length > 1) {
            toast.error(
                { message: "Please select only one image" },
                { style: "plain", theme: getTheme() },
            );
            return;
        }

        const file = elt.files[0];

        if (file.size > 1024 * 1024) {
            toast.error(
                { message: "File size must be less than 1MB." },
                { style: "plain", theme: getTheme() },
            );
            return;
        }

        if (!mimeTypesAllowed.includes(file.type)) {
            toast.error(
                { message: "File type not supported." },
                {
                    style: "plain",
                    theme: getTheme(),
                },
            );
            return;
        }

        const dialog = document.querySelector(
            "dialog[data-cropper-modal]",
        ) as HTMLDialogElement;
        const fileReader = new FileReader();

        fileReader.addEventListener(
            "load",
            () => {
                const image = dialog.querySelector(
                    "img[data-img]",
                ) as HTMLImageElement;
                image.src = fileReader.result as string;

                const cropper = new Cropper(image, {
                    aspectRatio: 16 / 9,
                    viewMode: 1,
                    zoomable: false,
                    minCropBoxWidth: 120,
                    responsive: false,
                    crop(event) {},
                });

                dialog.addEventListener("close", () => {
                    cropper.destroy();
                    image.src = "";
                    fileReader.abort();
                    elt.value = "";
                }, { once: true });

                dialog?.showModal();
                // Todo open modal
            },
            { once: true },
        );

        fileReader.readAsDataURL(file);
    });
});
