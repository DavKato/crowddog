let state: State | undefined = $state();

export const store = {
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

	add_to_history(work_content: WorkContent) {
		if (!is_initialized(state)) throw new UninitializedError();
		const { id: project_id } = work_content.project;
		const { id: process_id } = work_content.process;
		if (state.history.some((h) => h.project.id === project_id && h.process.id === process_id))
			return;

		state.history.unshift(work_content);
	},

	change_status(status: StopWatchStatus) {
		if (!is_initialized(state)) throw new UninitializedError();
		state.stop_watch.status = status;
	},
};

function is_initialized(s: typeof state): s is State {
	return s !== undefined;
}

class UninitializedError extends Error {
	constructor() {
		super('State is not initialized');
		this.name = 'Uninitialized';
	}
}
