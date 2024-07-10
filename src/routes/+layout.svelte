<script>
	import '../assets/reset.css';
	let { children } = $props();
</script>

<main>
	<div class="container">
		{@render children()}
	</div>
</main>

<style>
	:global(:root) {
		--color-primary: #0070f3;
		--color-secondary: #ff0080;
		--color-tertiary: #79ff97;
		--color-text: #fff;
		--color-bg: #333;
		--color-shadow: #fc0;
	}
	:global(html) {
		font-family: 'Noto Sans JP', sans-serif;
		font-size: clamp(0.8rem, 4vw + 0.1rem, 1.2rem);
		color: var(--color-text);
		background-color: var(--color-bg);
	}

	main {
		height: 100svh;
		max-width: 900px;
		margin: 0 auto;
		padding: 0 1rem;

		display: grid;
		grid-template-rows: 1fr auto 2fr;
		grid-template-columns: 1fr minmax(0, 900px) 1fr;
		grid-template-areas: '. . .' '. content .' '. . .';
	}
	.container {
		grid-area: content;
	}
</style>
