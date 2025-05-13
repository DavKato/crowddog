import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { store, TIMER_STATUS } from './store.svelte';

const EVENT = {
	TIMER_TICK: 'timer_tick',
};

// Wrapper of invoke. It automatically re-logins and retry the command when the original command failed because of session expiration.
const command = async <T>(...args: Parameters<typeof invoke>) => {
	try {
		const res = await invoke<T>(...args);
		return res;
	} catch (e) {
		const err = e as App.Error;
		if (err.status === 401) {
			await invoke('re_login');
			return invoke<T>(...args);
		}
		console.error(e);
		throw e;
	}
};

export const login = (credentials: Credentials) => {
	return invoke<void>('login', { credentials });
};

export const init_data = async () => {
	const res = await command<State>('init_data');
	if (res.stop_watch.status === TIMER_STATUS.STARTED) {
		start_timer(res.stop_watch);
	}
	return {
		...res,
		stop_watch: deserialize_sw(res.stop_watch),
	};
};

let my_patterns: Array<DefinedWorkContent> | undefined;
export const get_my_patterns = async () => {
	if (!my_patterns) {
		my_patterns = await command<DefinedWorkContent[]>('get_my_patterns');
	}

	return my_patterns;
};

export const get_processes = (project_id: DefinedWorkContent['project']['id']) => {
	return command<Process[]>('get_processes', { user_id: store.state.user.id, project_id });
};

export const set_work_content = async (
	project_id: DefinedWorkContent['project']['id'],
	process_id: DefinedWorkContent['process']['id'],
) => {
	const wc = await command<WorkContent>('set_work_content', {
		stop_watch: store.state.stop_watch,
		project_id,
		process_id,
	});
	store.update_work_content(wc);
};

let unlisten = () => {};
export const start_timer = async (stop_watch: StopWatch) => {
	const res = await command<StopWatch>('start_timer', { stop_watch });
	const sw = deserialize_sw(res);
	unlisten = await listen<string>(EVENT.TIMER_TICK, (e) => {
		store.setClock(e.payload);
	});
	store.update_stop_watch(sw);
	return sw;
};

export const stop_timer = async (stop_watch: StopWatch) => {
	unlisten();
	const res = await command<StopWatch>('stop_timer', { stop_watch });
	const sw = deserialize_sw(res);
	store.update_stop_watch(sw);
	store.setClock('00:00:00');
	return sw;
};

export const cancel_timer = async (stop_watch: StopWatch) => {
	unlisten();
	const res = await command<StopWatch>('cancel_timer', { stop_watch });
	const sw = deserialize_sw(res);
	store.update_stop_watch(sw);
	store.setClock('00:00:00');
	return sw;
};

const deserialize_sw = (sw: StopWatch): StopWatch => {
	if (!is_valid_project(sw.work_content.project)) sw.work_content.project = undefined;
	if (!is_valid_project(sw.work_content.process)) sw.work_content.process = undefined;
	console.log({ sw });
	return sw;
};

const is_valid_project = (t: WorkContent['project' | 'process']) => t && t.id > 0;
