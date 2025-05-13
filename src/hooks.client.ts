import { store } from '$lib/store.svelte';
import type { HandleClientError } from '@sveltejs/kit';

export const handleError: HandleClientError = ({ error, status }) => {
	store.clear_loading();

	const err = error as App.Error;

	return {
		status,
		message: err.message ?? 'Whoops',
		source: err.source,
	};
};
