import { toast } from "@webdevaaron/vanilla-toast";
import Cropper from "cropperjs";
// Also implement drop listener

const elt = document.getElementById("attach-img") as HTMLInputElement;

elt?.addEventListener("change", (evt) => {
    if (elt.files.length === 0) {
        return;
    }

    if (elt.files.length > 1) {
        toast.error({ message: "Please select only one image" });
        return;
    }

    const file = elt.files[0];

    if (file.size > 1024 * 1024) {
        toast.error({ message: "File size must be less than 1MB." });
        return;
    }

    const targetImageId = elt.getAttribute("data-target-image-id");
    const fileReader = new FileReader();

    fileReader.addEventListener("load", () => {
        const image = document.getElementById(targetImageId) as HTMLImageElement;
        image.src = fileReader.result as string;

        const cropper = new Cropper(image, {
            aspectRatio: 1,
            viewMode: 1,
            zoomable: false,
            minCropBoxWidth: 120,
            crop(event) {
                console.log(event.detail.x);
                console.log(event.detail.y);
                console.log(event.detail.width);
                console.log(event.detail.height);
                console.log(event.detail.rotate);
                console.log(event.detail.scaleX);
                console.log(event.detail.scaleY);
            },
        });

        // Todo open modal

        
    }, { once: true });

    fileReader.readAsDataURL(file);
});
