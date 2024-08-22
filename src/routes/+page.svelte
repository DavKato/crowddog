<script lang="ts">
	import { store, TIMER_STATUS } from '$lib/store.svelte';
	import { start_timer, stop_timer } from '$lib/io.svelte';
	import { goto } from '$app/navigation';
	import StopWatch from './StopWatch.svelte';

	let state = $derived(store.state);
	$inspect(state);

	let needApplication = $derived(state.stop_watch.status === TIMER_STATUS.NEED_TO_APPLY);
	$effect(() => {
		if (needApplication) goto('/need_application');
	});

	const on_time_clicked = async () => {
		store.set_loading_msg('');
		try {
			state.stop_watch.status === TIMER_STATUS.STOPPED
				? await start_timer(state.stop_watch)
				: await stop_timer(state.stop_watch);
		} finally {
			store.clear_loading();
		}
	};
</script>

<div class="contents">
	<StopWatch stop_watch={state.stop_watch} onmousedown={on_time_clicked}></StopWatch>
</div>

<style>
	.contents {
		display: grid;
		place-items: center;
		height: 100%;
	}
</style>
