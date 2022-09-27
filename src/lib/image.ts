import { get } from "svelte/store"
import { user_image } from "./stores/user";
import default_profile_image from "../assets/profile/default_profile.webp";

export class ImageLib {

    static getProfileImageUrl(): string {
        const imageValue = get(user_image);
        if (imageValue == null || imageValue == "null") {
            return default_profile_image;
        }

        return this.base64ToImage(imageValue).src;
    }

    /**
     * @param image the image to convert
     * @returns the image as a base64 string
     */
    static imageToBase64(image: HTMLImageElement): string {
        const canvas = document.createElement('canvas');
        canvas.width = image.width;
        canvas.height = image.height;
        const ctx = canvas.getContext('2d');
        if (ctx) {
            ctx.drawImage(image, 0, 0);
            return canvas.toDataURL('image/png');
        }
        return "";
    }

    /**
     * @param base64 the base64 string to convert
     * @returns the base64 string as an image
     * @example
     */
    static base64ToImage(base64: string): HTMLImageElement {
        const img = new Image();
        img.src = base64;
        return img;
    }
}