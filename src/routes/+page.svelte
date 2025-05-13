<script lang="ts">
	import { goto } from '$app/navigation';
	import { store, TIMER_STATUS } from '$lib/store.svelte';
	import { start_timer, stop_timer, cancel_timer } from '$lib/io.svelte';
	import { ChevronRight } from '$lib/icons';
	import StopWatch from './StopWatch.svelte';

	let { state } = store;
	const { project, process } = state.stop_watch.work_content;
	$inspect(state);

	let needApplication = $derived(state.stop_watch.status === TIMER_STATUS.NEED_TO_APPLY);
	$effect(() => {
		if (needApplication) goto('/need_application');
	});

	const select_wc = (prj_id?: number) => {
		let url = '/set_wc';
		if (prj_id) url += `/normal/${prj_id}`;
		goto(url);
	};

	const on_timer_clicked = async () => {
		store.set_loading_msg('');
		try {
			state.stop_watch.status === TIMER_STATUS.STOPPED
				? await start_timer(state.stop_watch)
				: await stop_timer(state.stop_watch);
		} finally {
			store.clear_loading();
		}
	};
	const on_cancel_clicked = async () => {
		store.set_loading_msg('');
		try {
			await cancel_timer(state.stop_watch);
		} finally {
			store.clear_loading();
		}
	};
</script>

<div class="contents">
	<section class="wc">
		<button class="select ring_inset" onclick={() => select_wc()}>{project?.name ?? '-'}</button
		>
		<span class="mid"><ChevronRight /></span>
		<button class="select ring_inset" onclick={() => select_wc(project?.id)}
			>{process?.name ?? '-'}</button
		>
	</section>

	<StopWatch stop_watch={state.stop_watch} {on_timer_clicked} {on_cancel_clicked}></StopWatch>
</div>

<style>
	.contents {
		display: grid;
		grid-template-rows: 5rem 1fr;
		align-items: start;
		height: 100%;
	}

	.wc {
		overflow: hidden;
		width: 100%;
		height: 100%;

		display: grid;
		align-items: stretch;
		justify-content: space-between;
		grid-template-columns: 1fr auto 1fr;
		gap: 0.3rem;

		background-color: var(--color-bg);
		color: var(--color-text);
		border-top: 1px solid var(--color-text);
		border-bottom: 1px solid var(--color-text);
	}

	.wc > span {
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.wc > .select {
		padding: 1rem 0.3rem;
		text-align: center;
		transition:
			background-color 0.2s,
			color 0.3s;
	}
	.wc > .select:hover {
		color: var(--color-bg);
		background-color: var(--color-text);
	}
</style>
