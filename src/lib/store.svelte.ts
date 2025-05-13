let state: State | undefined = $state();
let loading_msg: string | undefined = $state();
let is_loading: boolean = $derived(loading_msg !== undefined);
let clock: string = $state('00:00:00');

export const TIMER_STATUS = {
	STARTED: 'timing',
	STOPPED: 'reset',
	NEED_TO_APPLY: 'stopped',
} as const;

export const store = $state({
	get is_loading() {
		return is_loading;
	},

	get loading_msg() {
		return loading_msg;
	},

	set_loading_msg(msg: string) {
		loading_msg = msg;
	},

	clear_loading() {
		loading_msg = undefined;
	},

	get state() {
		if (!is_initialized(state)) throw new UninitializedError();
		return state;
	},

	try_get_state() {
		return state;
	},

	init(d: State) {
		state = d;
	},

	clear() {
		state = undefined;
	},

	is_initialized() {
		return is_initialized(state);
	},

	update_work_content(wc: WorkContent) {
		if (!is_initialized(state)) throw new UninitializedError();
		state.stop_watch.work_content = wc;
		this.add_to_history(wc);
	},

	add_to_history(work_content: WorkContent) {
		if (!is_initialized(state)) throw new UninitializedError();
		if (!is_defined_work_content(work_content)) throw new UndefinedWorkContentError();
		const { id: project_id } = work_content.project;
		const { id: process_id } = work_content.process;
		if (state.history.some((h) => h.project.id === project_id && h.process.id === process_id))
			return;

		state.history.unshift(work_content);
	},

	update_stop_watch(stop_watch: StopWatch) {
		if (!is_initialized(state)) throw new UninitializedError();
		state.stop_watch = stop_watch;
	},

	get clock() {
		return clock;
	},

	setClock(c: string) {
		clock = c;
	},
});

const is_initialized = (s: typeof state): s is State => {
	return s !== undefined;
};

class UninitializedError extends Error {
	constructor() {
		super('State is not initialized');
		this.name = 'Uninitialized';
	}
}

export const is_defined_work_content = (t: WorkContent): t is DefinedWorkContent => {
	return t.project !== undefined && t.process !== undefined;
};

class UndefinedWorkContentError extends Error {
	constructor() {
		super('Either or both of PROJECT or PROCESS is not defined');
		this.name = 'UndefinedWorkContent';
	}
}
