import { invoke } from "@tauri-apps/api/tauri";

export const setAuth = (user: User) => {
    return invoke('login', { newAuth: user });
}

export const initData = (): Promise<Data> => {
    return invoke('init_data');
}

type Data = Record<string, any>;
