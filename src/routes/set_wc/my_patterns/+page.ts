import { get_my_patterns, get_processes } from '$lib/io.svelte.js';
import { store } from '$lib/store.svelte';

export async function load() {
	store.set_loading_msg('');
	const my_patterns = await get_my_patterns();
	store.clear_loading();

	return {
		my_patterns,
	};
}
