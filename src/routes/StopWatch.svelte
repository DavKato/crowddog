<script lang="ts">
	import { TIMER_STATUS, store } from '$lib/store.svelte';
	let { stop_watch, onmousedown, ...rest }: { stop_watch: StopWatch; onmousedown: () => void } =
		$props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width="100%"
	viewBox="0 0 24 24"
	{...rest}
	role="button"
	tabindex="0"
	{onmousedown}
>
	<g fill="none" stroke="currentColor" stroke-width="1.5">
		<circle cx="12" cy="13" r="9" />
		<path stroke-linecap="round" d="M10 2h4" />
		{#if stop_watch.status === TIMER_STATUS.STOPPED}
			<path
				class="status"
				d="M13.888 10.935C14.963 11.812 15.5 12.25 15.5 13s-.537 1.188-1.612 2.065c-.297.242-.591.47-.862.66c-.237.167-.506.339-.784.508c-1.073.652-1.609.978-2.09.617c-.48-.36-.524-1.116-.612-2.628c-.024-.427-.04-.846-.04-1.222s.016-.795.04-1.222c.088-1.512.132-2.267.612-2.628c.481-.361 1.018-.035 2.09.617c.278.169.547.341.784.508c.27.19.565.418.862.66Z"
			/>
		{:else if stop_watch.status === TIMER_STATUS.STARTED}
			<text x="6" y="11" textLength="12" lengthAdjust="spacingAndGlyphs" stroke-width="0"
				>{store.clock}</text
			>
			<rect class="status" x="10" y="13" width="4" height="4" fill="currentColor" />
		{:else}
			?{stop_watch.status}
		{/if}
	</g>
</svg>

<style>
	svg {
		width: 80%;
		color: var(--color-primary);
		border-radius: 50%;
		cursor: pointer;
		transition: filter 0.2s;
	}
	svg:hover {
		filter: drop-shadow(
			0.1rem 0.1rem 0.7rem color-mix(in srgb, var(--color-primary) 28%, transparent)
		);
	}
	svg .status {
		transition: scale 0.2s ease-out;
		transform-origin: center;
	}
	svg:hover .status {
		scale: 0.95;
	}
	svg:active .status {
		scale: 0.925;
	}
	svg text {
		font-size: 3px;
		font-weight: 100;
		fill: var(--color-text);
	}
	svg:focus {
		outline: none;
	}
</style>
