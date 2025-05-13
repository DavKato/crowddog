export const ssr = false;
export const prerender = false;

import { store } from '$lib/store.svelte';
import { redirect } from '@sveltejs/kit';

export function load({ url }) {
	if (url.pathname !== '/login' && !store.is_initialized())
		return redirect(307, '/login?logged_in=true');
}
