import { writable } from "svelte/store";
import { invoke } from  '@tauri-apps/api/tauri';

let user_name = writable("null");
let user_position = writable("null");
let user_level = writable(0);
let user_status = writable("null");
let user_discord = writable("null");
let user_image = writable("null");

type UserUpdateOptions = {
    status?: "online" | "away" | "offline";
    discord?: string | null;
    image?: string | null;
}

/**
 * updates the store values with the given options
 * it also updates them in the database
 * @param option what to update with new values
 * @example updateUserData({ position: "developer" })
 */
async function updateUserData(option: UserUpdateOptions) {
    if (option.status) {
        user_status.set(option.status);
        await invoke("update_user_status", { status: option.status });
    }
    if (option.discord) {
        user_discord.set(option.discord);
        await invoke("update_user_discord", { discord: option.discord });
    }
    if (option.image) {
        user_image.set(option.image);
        await invoke("update_user_image", { image: option.image });
    }
}

export { user_name, user_position, user_status, user_discord, user_image, user_level };