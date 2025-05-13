export const KEYBOARD = {
	UP: 'ArrowUp',
	DOWN: 'ArrowDown',
	SPACE: ' ',
	ENTER: 'Enter',
};

export const is_submittive_key = (key: KeyboardEvent['key']) =>
	key === KEYBOARD.SPACE || key === KEYBOARD.ENTER;
