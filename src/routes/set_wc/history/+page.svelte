<script lang="ts">
	import { goto } from '$app/navigation';
	import { store } from '$lib/store.svelte';
	import { set_work_content } from '$lib/io.svelte.js';
	import { context } from '../context.svelte.js';
	import Selector from '../Selector.svelte';

	context.title = 'Select from history';

	const list = store.state.history.map((item, i) => ({
		id: i + 1,
		name: `${item.project.name}  ▶️  ${item.process.name}`,
		...item,
	}));

	const onselect = async (item: (typeof list)[number]) => {
		store.set_loading_msg('');
		const { project, process } = list[item.id - 1];
		await set_work_content(project.id, process.id);
		await goto('/');
		store.clear_loading();
	};
</script>

<Selector {list} {onselect} />
