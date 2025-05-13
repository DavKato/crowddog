<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { store } from '$lib/store.svelte';
	import { context } from '../context.svelte';
	import Selector from '../Selector.svelte';

	context.title = 'Select a project';

	const onselect = async (pj: Project) => {
		store.set_loading_msg('');
		console.log(page.url.pathname + `/${pj.id}`);
		await goto(page.url.pathname + `/${pj.id}`);
		store.clear_loading();
	};
</script>

<Selector list={store.state.projects} {onselect} />
