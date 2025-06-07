<script lang="ts">
	import { goto } from '$app/navigation';
	import { login as _login, init_data } from '$lib/io.svelte';
	import { Eye } from '$lib/icons';
	import { onMount } from 'svelte';
	import { store } from '$lib/store.svelte';

	let { data } = $props();

	let is_logged_in = $state(data.is_logged_in);
	let email = $state('');
	let passwd = $state('');
	let show_passwd = $state(false);
	let is_filled = $derived(email && passwd);
	let error_message = $state('');

	onMount(() => {
		if (data.is_logged_in) submit();
	});

	async function login() {
		store.set_loading_msg('');
		error_message = '';
		await _login({ email, passwd });
		is_logged_in = true;
	}

	async function init() {
		store.set_loading_msg('Login successful. Initializing data...');
		const res = await init_data();
		store.init(res);
		goto('/', { replaceState: true });
		store.clear_loading();
	}

	async function submit() {
		try {
			if (!is_logged_in) await login();
			await init();
		} catch (e) {
			console.error(e);
			is_logged_in = false;

			const err = e as App.Error;
			if (err.status !== 400) throw err;
			error_message = err.message;
			store.clear_loading();
		}
	}
</script>

<div class="container">
	<fieldset inert={store.is_loading}>
		<legend>Input your credentials for CrowdLog.</legend>
		<label>
			<span>EMAIL</span>
			<input autofocus type="text" bind:value={email} />
		</label>
		<label style="position: relative">
			<span>PASSWORD</span>
			<input type={show_passwd ? 'text' : 'password'} bind:value={passwd} />
			<Eye
				class="pw-icon"
				hidden={!show_passwd}
				onclick={() => (show_passwd = !show_passwd)}
			/>
		</label>
		<button disabled={!is_filled} onclick={submit}>OK</button>

		<p class="error">{error_message}</p>
	</fieldset>
</div>

<style>
	.container {
		height: 100%;
		display: grid;
		place-items: center;
	}
	fieldset {
		position: relative;
		height: min(19rem, 100svh);
		width: min(26rem, 100%);
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		align-items: start;
		justify-content: center;

		border: 2px solid var(--color-text);
		border-radius: 10px;
		padding: 1rem 2rem;
	}
	legend {
		padding: 0 0.6rem;
		margin-left: -1.5rem;
		margin-bottom: 1rem;
		background: linear-gradient(
			to right,
			transparent,
			var(--color-bg) 3%,
			var(--color-bg) 97%,
			transparent
		);
		font-style: italic;
		word-break: keep-all;
	}
	label {
		width: 100%;
		margin-bottom: 2rem;
	}
	label:focus-within,
	label:active {
		text-shadow: 2px 1px 4px var(--color-shadow);
	}
	label span {
		user-select: none;
	}
	label input {
		width: 100%;
		padding: 0.5rem 1.8rem 0.3rem 0.4rem;
		border-bottom: 1px solid var(--color-text);
	}
	label :global(.pw-icon) {
		position: absolute;
		right: 0.5rem;
		top: calc(50% + 0.6rem);
		transform: translateY(-50%);
		cursor: pointer;
	}

	button {
		align-self: center;
		padding: 0.7rem 1.8rem;
		border-radius: 10px;
		border: 1px solid var(--color-text);
		box-shadow: 0px 0px 0px var(--color-shadow);
		transition:
			box-shadow 0.15s,
			border-color 0.15s;
		user-select: none;
	}
	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
	button:not(:disabled):hover,
	button:not(:disabled):focus {
		border-color: var(--color-shadow);
		box-shadow:
			2px 2px 5px var(--color-shadow),
			inset 1px 1px 3px var(--color-shadow);
	}
	button:not(:disabled):active {
		border-color: var(--color-shadow);
		box-shadow:
			1px 1px 1px var(--color-shadow),
			inset 1px 1px 1px var(--color-shadow);
	}

	.error {
		position: absolute;
		top: 90%;
		left: 0%;
		width: 100%;
		text-align: center;
		color: var(--color-danger);
	}
</style>
