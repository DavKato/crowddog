<script>
	import { draw } from 'svelte/transition';
	import { quadInOut } from 'svelte/easing';
	let { hidden, ...props } = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" {...props}>
	<path
		fill="currentColor"
		d="M12 9a3 3 0 0 0-3 3a3 3 0 0 0 3 3a3 3 0 0 0 3-3a3 3 0 0 0-3-3m0 8a5 5 0 0 1-5-5a5 5 0 0 1 5-5a5 5 0 0 1 5 5a5 5 0 0 1-5 5m0-12.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5"
	/>
	{#if hidden}
		<line
			transition:draw={{ duration: 100, easing: quadInOut }}
			x1="2"
			y1="2"
			x2="22"
			y2="22"
			stroke="currentColor"
			stroke-width="3"
		/>
	{/if}
</svg>
