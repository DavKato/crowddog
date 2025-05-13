<script lang="ts">
	import { page } from '$app/state';

	let { status, message, source } = page.error!;
</script>

<div class="wrapper">
	<h1>Something went wrong😢</h1>

	<div class="details">
		<p class="status">status: {status}</p>
		<hr />
		<p class="message">{message}</p>
		{#if source}
			<pre class="source">{source}</pre>
		{/if}
	</div>

	<footer>
		<p>Please contact Davide with this screenshot.</p>
		<a href="/" class="go_top">Back to TOP</a>
	</footer>
</div>

<style>
	.wrapper {
		height: min(100%, 400px);
		padding: 16px 8px;
		display: grid;
		grid-template-rows: auto minmax(0, 1fr) auto;
		line-height: 1.5;
	}
	h1 {
		font-size: 1.6rem;
	}

	.details {
		align-self: center;
		padding: 8px 16px;
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
		border: 1px solid var(--color-shadow);
	}
	p {
		font-size: 1.2rem;
		font-weight: normal;
	}

	hr {
		border-top: 1px solid var(--color-shadow);
	}

	.source {
		margin: 0;
		white-space: initial;
		padding: 8px 16px;
		background-color: var(--color-bg);
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-items: flex-end;
		gap: 0.4rem;

		a {
			padding: 8px 16px;
			align-self: flex-end;
			border: 1px solid var(--color-bg);
			border-radius: 5px;
			transition:
				background-color 0.2s,
				color 0.3s;

			&:hover {
				color: var(--color-bg);
				background-color: var(--color-text);
			}
		}
	}
</style>
