<script lang="ts">
	let { ...props } = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" {...props}>
	<g fill="none" stroke="currentColor">
		<path d="M15 3.5h5.5m0 0V9m0-5.5l-8 8" />
		<path
			stroke-linecap="round"
			d="M11.5 5.5h-4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2v-4"
		/>
	</g>
</svg>
