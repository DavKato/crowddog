<script lang="ts">
	import { TIMER_STATUS, is_defined_work_content, store } from '$lib/store.svelte';
	let {
		stop_watch,
		on_timer_clicked,
		on_cancel_clicked,
		...rest_props
	}: { stop_watch: StopWatch; on_timer_clicked: Noop; on_cancel_clicked: Noop } = $props();

	let is_started = $derived(stop_watch.status === TIMER_STATUS.STARTED);
	let is_stopped = $derived(stop_watch.status === TIMER_STATUS.STOPPED);

	let timer: HTMLButtonElement;
	$effect(() => {
		if (is_started || is_defined_work_content(stop_watch.work_content)) timer.focus();
	});
</script>

<div class="wrapper" {...rest_props}>
	<button bind:this={timer} class="timer" onclick={on_timer_clicked}>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="100%"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
		>
			<circle cx="12" cy="13" r="9" />
			<path stroke-linecap="round" d="M10 2h4" />
			{#if is_stopped}
				<path
					class="status"
					d="M13.888 10.935C14.963 11.812 15.5 12.25 15.5 13s-.537 1.188-1.612 2.065c-.297.242-.591.47-.862.66c-.237.167-.506.339-.784.508c-1.073.652-1.609.978-2.09.617c-.48-.36-.524-1.116-.612-2.628c-.024-.427-.04-.846-.04-1.222s.016-.795.04-1.222c.088-1.512.132-2.267.612-2.628c.481-.361 1.018-.035 2.09.617c.278.169.547.341.784.508c.27.19.565.418.862.66Z"
				/>
			{:else if is_started}
				<text x="6" y="11" textLength="12" lengthAdjust="spacingAndGlyphs" stroke-width="0"
					>{store.clock}</text
				>
				<rect class="status" x="10" y="13" width="4" height="4" fill="currentColor" />
			{:else}
				?{stop_watch.status}
			{/if}
		</svg>
	</button>
	{#if is_started}
		<button class="cancel" onclick={on_cancel_clicked} aria-label="cancel">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="3.5em"
				height="3.5em"
				viewBox="0 0 24 24"
				fill="currentColor"
			>
				<path
					d="M12 2c5.5 0 10 4.5 10 10s-4.5 10-10 10S2 17.5 2 12S6.5 2 12 2m0 2c-1.9 0-3.6.6-4.9 1.7l11.2 11.2c1-1.4 1.7-3.1 1.7-4.9c0-4.4-3.6-8-8-8m4.9 14.3L5.7 7.1C4.6 8.4 4 10.1 4 12c0 4.4 3.6 8 8 8c1.9 0 3.6-.6 4.9-1.7"
				/>
			</svg>
		</button>
	{/if}
</div>

<style>
	.wrapper {
		position: relative;
		align-self: center;
		width: 100%;
	}
	button {
		display: block;
		transition: filter 0.2s;
		border-radius: 50%;
	}
	button:hover,
	button:focus-visible {
		filter: drop-shadow(0 0 0.5rem color-mix(in srgb, currentColor 60%, transparent));
		outline-color: color-mix(in srgb, currentColor 40%, transparent);
	}
	.timer {
		width: 60%;
		margin: 0 auto;
		color: var(--color-primary);
	}
	.timer .status {
		transition: scale 0.2s ease-out;
		transform-origin: center;
	}
	.timer:hover .status,
	.timer:focus-visible .status {
		scale: 0.95;
	}
	.timer:active .status {
		scale: 0.925;
	}
	.timer text {
		font-size: 3px;
		font-weight: 100;
		fill: var(--color-text);
	}

	button.cancel {
		position: absolute;
		bottom: 0%;
		right: 10%;
		color: var(--color-danger);
		scale: 0.95;
		transition:
			filter 0.2s,
			scale 0.2s;
	}
	button.cancel:hover {
		scale: 1;
	}
</style>
