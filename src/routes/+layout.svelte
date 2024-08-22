<script>
	import '../assets/reset.css';
	import { page } from '$app/stores';
	import { store } from '$lib/store.svelte';
	import { Logout, ExternalLink } from '$lib/icons';
	import { goto } from '$app/navigation';
	import LoadingOverlay from './LoadingOverlay.svelte';

	let { children } = $props();
	let is_login_page = $derived($page.url.pathname === '/login');
	let email = $derived(store.try_get_state()?.user.email ?? '');

	function logout() {
		// TODO:
		goto('/login');
	}
</script>

<div class="page">
	<header>
		{#if !is_login_page}
			<span aria-hidden="true"></span>
			<span>{email}</span>
			<button title="logout" onclick={logout}><Logout></Logout></button>
		{/if}
	</header>
	<main>
		{@render children()}
	</main>

	<footer>
		<a href="https://app.crowdlog.jp/login.cgi" target="_blank">
			<span>CrowdLog</span>
			<span class="icon"><ExternalLink></ExternalLink></span>
		</a>
	</footer>

	<LoadingOverlay></LoadingOverlay>
</div>

<style>
	:global(:root) {
		--color-primary: #306cfe;
		--color-danger: #ff0080;
		--color-tertiary: #79ff97;
		--color-text: #fff;
		--color-text-dark: #ccc;
		--color-bg: #333;
		--color-bg-dark: #202020;
		--color-shadow: #fc0;
	}
	:global(html) {
		font-family: 'Noto Sans JP', sans-serif;
		font-size: clamp(0.8rem, 4vw + 0.1rem, 1.2rem);
		color: var(--color-text);
		background-color: var(--color-bg);
	}

	.page {
		height: 100svh;
		overflow: hidden;

		display: grid;
		grid-template-rows: 1.8rem 1fr minmax(auto, 1.4rem);
		grid-template-columns: 1fr minmax(0, 900px) 1fr;
		grid-template-areas: 'header header header' '. content .' 'footer footer footer';

		--bdw: 0.6rem;
		border-right: var(--bdw) solid var(--color-bg-dark);
		border-left: var(--bdw) solid var(--color-bg-dark);
	}
	header {
		grid-area: header;
		width: 100%;
		height: 100%;
		font-size: 0.8rem;

		display: flex;
		align-items: center;
		justify-content: flex-end;

		background-color: var(--color-bg-dark);
	}
	header > button {
		height: 100%;
		aspect-ratio: 1;
		display: grid;
		place-items: center;
		font-size: 1.1rem;
		cursor: pointer;
	}
	main {
		grid-area: content;
	}

	footer {
		grid-area: footer;
		width: 100%;
		height: 100%;
		font-size: 0.8rem;

		display: flex;
		align-items: center;
		justify-content: flex-end;

		background-color: var(--color-bg-dark);
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
