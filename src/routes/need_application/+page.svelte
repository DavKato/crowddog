<script lang="ts">
	import { store } from '$lib/store.svelte';
	import { init_data } from '$lib/io.svelte';
	import { goto } from '$app/navigation';

	const reload = async () => {
		store.set_loading_msg('Checking stop watch status...');
		const res = await init_data();
		store.init(res);
		goto('/');
		store.clear_loading();
	};
</script>

<div class="container">
	<h1>[Need Action🤷‍♂️]</h1>
	<p>Timer has been stopped without applying a work content.</p>
	<p>
		Fix it in <a href="https://app.crowdlog.jp/pages/timesheet/stop_watch/" target="_blank"
			>their stop watch page</a
		> and then click the button below.
	</p>

	<button onmousedown={reload}>FIXED!</button>
</div>

<style>
	.container {
		height: 100%;
		padding: 15% 2rem;
		display: flex;
		flex-direction: column;
		gap: 1.4rem;
		font-size: 1.2rem;
	}
	h1 {
		color: var(--color-shadow);
	}
	p {
		line-height: 1.5;
	}
	a {
		font-style: italic;
		text-decoration: underline;
		transition: opacity 0.2s;
	}
	a:hover {
		opacity: 0.8;
	}
	button {
		align-self: flex-end;
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		background-color: var(--color-primary);
		color: var(--color-text);
		border: none;
		border-radius: 5px;
	}
</style>
