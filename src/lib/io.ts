import { invoke } from "@tauri-apps/api/tauri";

export const setAuth = (user: User) => {
    return invoke('login', { newAuth: user });
}
