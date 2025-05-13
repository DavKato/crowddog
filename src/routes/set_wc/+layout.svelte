<script lang="ts">
	import { ChevronRight } from '$lib/icons';
	import { context } from './context.svelte';
	import { Star, History, List } from '$lib/icons';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { children } = $props();

	const change_mode = (mode: typeof context.mode) => {
		context.mode = mode;
		goto(mode + page.url.search);
	};
</script>

<div class="title">
	<button aria-label="go back" onclick={() => goto('/')}
		><ChevronRight style="rotate: 180deg" /></button
	>
	<legend>{context.title}</legend>
</div>

<nav>
	<button
		class={{ active: context.mode === 'history', ring_inset: true }}
		onclick={() => change_mode('history')}><History /></button
	>
	<button
		class={{ active: context.mode === 'my_patterns', ring_inset: true }}
		onclick={() => change_mode('my_patterns')}><Star /></button
	>
	<button
		class={{ active: context.mode === 'normal', ring_inset: true }}
		onclick={() => change_mode('normal')}><List /></button
	>
</nav>

{@render children()}

<style>
	.title {
		height: 3rem;
		padding: 0 0.4rem;
		display: flex;
		align-items: center;
		gap: 0.2rem;
		font-size: 1.2rem;

		> * {
			padding: 0.4rem;
		}
		> button {
			display: flex;
			align-items: center;
		}
	}

	nav {
		height: 2.4rem;
		display: flex;
		justify-content: space-evenly;
		align-items: center;
		border-top: 1px solid var(--color-bg);
		border-bottom: 1px solid var(--color-bg);
		font-size: 1.5rem;

		button {
			width: 100%;
			height: 100%;
			display: flex;
			justify-content: center;
			align-items: center;
			color: var(--color-shadow);
			transition: color 0.2s;

			&:not(.active):hover {
				color: var(--color-text);
			}
			&.active {
				background-color: var(--color-bg);
			}
		}
	}
</style>
