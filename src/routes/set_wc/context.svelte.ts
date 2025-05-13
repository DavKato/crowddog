type Mode = 'normal' | 'history' | 'my_patterns';

const _context = $state({
	title: '',
	mode: (localStorage.getItem('mode') ?? 'history') as Mode,
});

export const context = {
	get title() {
		return _context.title;
	},
	set title(t: string) {
		_context.title = t;
	},
	get mode() {
		return _context.mode;
	},
	set mode(m: Mode) {
		_context.mode = m;
		localStorage.setItem('mode', m);
	},
};
