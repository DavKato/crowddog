type ValueOf<T> = T[keyof T];

type Credentials = {
	email: string;
	passwd: string;
};

type User = {
	id: number;
	email: string;
	name: string;
};

type StopWatchStatus = ValueOf<typeof import('./store.svelte').TIMER_STATUS>;
type StopWatch = {
	id: number;
	start_at: string;
	status: StopWatchStatus;
};

type Project = {
	id: number;
	name: string;
};

type Process = {
	id: number;
	name: string;
};

type WorkContent = {
	project: Project;
	process: Process;
};

type State = {
	user: User;
	stop_watch: StopWatch;
	projects: Project[];
	history: WorkContent[];
};
