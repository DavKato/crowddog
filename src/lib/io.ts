import { invoke } from '@tauri-apps/api/tauri';

export const login = (credentials: Credentials) => {
	return invoke('login', { credentials });
};

export const init_data = (): Promise<State> => {
	return invoke('init_data');
};
