import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { store, TIMER_STATUS } from './store.svelte';

const EVENT = {
	TIMER_TICK: 'timer_tick',
};

export const login = (credentials: Credentials) => {
	return invoke<void>('login', { credentials });
};

export const init_data = async () => {
	const res = await invoke<State>('init_data');
	if (res.stop_watch.status === TIMER_STATUS.STARTED) {
		start_timer(res.stop_watch);
	}
	return res;
};

let unlisten = () => {};
export const start_timer = async (stop_watch: StopWatch) => {
	const sw = await invoke<StopWatch>('start_timer', { stop_watch });
	unlisten = await listen<String>(EVENT.TIMER_TICK, (e) => {
		store.setClock(e.payload);
	});
	store.update_stop_watch(sw);
	return sw;
};

export const stop_timer = async (stop_watch: StopWatch) => {
	unlisten();
	const sw = await invoke<StopWatch>('stop_timer', { stop_watch });
	store.update_stop_watch(sw);
	store.setClock('00:00:00');
	return sw;
};

export const cancel_timer = async (stop_watch: StopWatch) => {
	unlisten();
	const sw = await invoke<StopWatch>('cancel_timer', { stop_watch });
	store.update_stop_watch(sw);
	store.setClock('00:00:00');
	return sw;
};
