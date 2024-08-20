<script>
	import { fade } from 'svelte/transition';
	import { Loader } from '$lib/icons';
	import { store } from '$lib/store.svelte';
</script>

{#if store.is_loading}
	<div transition:fade class="cover">
		{#key store.loading_msg}
			<p in:fade>{store.loading_msg}</p>
		{/key}
		<Loader></Loader>
	</div>
{/if}

<style>
	.cover {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		font-size: 2rem;
		color: var(--color-text);
	}
	.cover p {
		font-size: 1rem;
	}
</style>
