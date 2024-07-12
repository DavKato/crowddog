import { invoke } from "@tauri-apps/api/tauri";

export const saveAuth = (user: User) => {
    return invoke('save_auth', { newAuth: user });
}
