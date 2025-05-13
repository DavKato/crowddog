<script lang="ts">
	import { goto } from '$app/navigation';
	import { store } from '$lib/store.svelte';
	import { set_work_content } from '$lib/io.svelte.js';
	import { context } from '../../context.svelte.js';
	import Selector from '../../Selector.svelte';

	let { data } = $props();
	const { project, processes } = data;

	context.title = project.name;

	const onselect = async (process: Process) => {
		store.set_loading_msg('');
		await set_work_content(project.id, process.id);
		await goto('/');
		store.clear_loading();
	};
</script>

<Selector list={processes} {onselect} />
