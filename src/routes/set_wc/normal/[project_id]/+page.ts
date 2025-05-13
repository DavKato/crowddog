import { redirect } from '@sveltejs/kit';
import { get_processes } from '$lib/io.svelte.js';
import { store } from '$lib/store.svelte.js';
import { context } from '../../context.svelte.js';

export async function load({ params }) {
	if (context.mode !== 'normal') redirect(307, `/set_wc/${context.mode}`);

	const project_id = Number(params.project_id);
	if (Number.isNaN(project_id)) redirect(307, '/set_wc');

	const project = store.state.projects.find((p) => p.id === project_id);
	if (project === undefined) redirect(307, '/set_wc');

	const processes = await get_processes(project_id);

	return {
		project,
		processes,
	};
}
