import { redirect } from '@sveltejs/kit';
import { context } from './context.svelte';

export const load = () => {
	const { mode } = context;
	redirect(307, `/set_wc/${mode}`);
};
