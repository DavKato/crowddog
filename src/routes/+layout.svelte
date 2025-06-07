<script>
	import '../assets/reset.css';
	import '../assets/base.css';
	import { page } from '$app/state';
	import { store } from '$lib/store.svelte';
	import { Logout, ExternalLink } from '$lib/icons';
	import { goto } from '$app/navigation';
	import LoadingOverlay from './LoadingOverlay.svelte';
	import { clear_data } from '$lib/io.svelte';

	let { children } = $props();
	let is_login_page = $derived(page.url.pathname === '/login');
	let email = $derived(store.try_get_state()?.user.email ?? '');

	const logout = async () => {
		await clear_data();
		store.clear();
		goto('/login');
	};
</script>

<div class="page">
	<header>
		{#if !is_login_page}
			<span aria-hidden="true"></span>
			<span>{email}</span>
			<button title="logout" class="ring_inset" onclick={logout}><Logout></Logout></button>
		{/if}
	</header>
	<main>
		{@render children()}
	</main>

	<footer>
		<a href="https://app.crowdlog.jp/login.cgi" target="_blank" class="ring_inset">
			<span>CrowdLog</span>
			<span class="icon"><ExternalLink></ExternalLink></span>
		</a>
	</footer>

	<LoadingOverlay></LoadingOverlay>
</div>

<style>
	.page {
		height: 100svh;
		overflow: hidden;

		display: grid;
		grid-template-rows: 1.8rem 1fr minmax(auto, 1.4rem);
		grid-template-columns: 1fr minmax(0, 900px) 1fr;
		grid-template-areas:
			'header header header'
			'. content .'
			'footer footer footer';

		--bdw: 0.6rem;
		border-right: var(--bdw) solid var(--color-bg);
		border-left: var(--bdw) solid var(--color-bg);
	}
	header {
		grid-area: header;
		width: 100%;
		height: 100%;
		font-size: 0.8rem;

		display: flex;
		align-items: center;
		justify-content: flex-end;

		background-color: var(--color-bg);
	}
	header > button {
		height: 100%;
		aspect-ratio: 1;
		display: grid;
		place-items: center;
		font-size: 1.1rem;
	}
	main {
		grid-area: content;
		overflow: hidden;
	}

	footer {
		grid-area: footer;
		width: 100%;
		height: 100%;
		font-size: 0.8rem;

		display: flex;
		align-items: center;
		justify-content: flex-end;

		background-color: var(--color-bg);
		color: var(--color-text-dark);
	}
	footer a {
		display: flex;
		align-items: center;
		user-select: none;
	}
	footer .icon {
		font-size: 1.1rem;
		display: contents;
	}
</style>
