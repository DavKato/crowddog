<script lang="ts" generics="T extends { id: number; name: string }">
	import { Search, Cancel } from '$lib/icons';
	import { KEYBOARD as KEY, is_submittive_key } from '$lib/utils';

	type Props = {
		list: Array<T>;
		onselect: (item: T) => void;
	};
	let { list: original_list, onselect }: Props = $props();

	let search_input = $state('');
	let list = $derived(
		original_list.filter((item) =>
			item.name.toLowerCase().includes(search_input.toLowerCase()),
		),
	);

	const focus_list = () => {
		const first_input = document.getElementById('list')?.getElementsByTagName('input')?.[0];
		if (!first_input) return;
		first_input.checked = true;
		first_input.focus();
	};
</script>

<fieldset>
	<div
		class="search"
		onkeydowncapture={(e) => {
			if (e.key === KEY.UP || e.key === KEY.DOWN) {
				focus_list();
			}
		}}
	>
		<Search />
		<input autofocus bind:value={search_input} aria-label="search by name" />
		<Cancel
			role="button"
			tabindex="0"
			class="clear_search"
			title="clear search"
			onclick={() => (search_input = '')}
		/>
	</div>

	<div id="list">
		{#each list as item (item.id)}
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
			<label onclick={() => onselect(item)}>
				<input
					type="radio"
					value={item.id}
					name="item"
					class="sr_only"
					onclick={(e) => e.stopPropagation()}
					onkeydown={(e) => {
						if (is_submittive_key(e.key)) onselect(item);
					}}
				/>
				<span>{item.name}</span>
			</label>
		{/each}
	</div>
</fieldset>

<style>
	fieldset {
		height: 100%;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.search {
		margin: 0.2rem 0.4rem 0.4rem;
		padding: 0.4rem 0.8rem;
		display: flex;
		gap: 0.4rem;
		align-items: center;
		background-color: var(--color-bg);
		border-radius: 4px;
		font-size: 1.2rem;

		input {
			flex: 1 1 0;
		}

		:global(.clear_search) {
			scale: 1.2;
			border-radius: 50%;
			transition:
				background-color 0.2s,
				color 0.3s;
			:global(&:hover) {
				color: var(--color-bg);
				background-color: var(--color-text);
			}
		}
	}

	#list {
		flex: 0 1 auto;
		overflow-y: auto;
		padding: 0.2rem 0;
	}

	label {
		display: block;
		width: 100%;
		padding: 0.4rem 1.2rem;
		white-space: pre;
		cursor: pointer;
		transition:
			background-color 0.2s,
			color 0.3s;

		&:has(input:checked),
		&:hover {
			background-color: var(--color-bg);
		}
		&:focus-within {
			outline: solid 2px var(--color-shadow);
		}

		&:not(:last-of-type) {
			border-bottom: 1px solid var(--color-bg);
		}
	}
</style>
