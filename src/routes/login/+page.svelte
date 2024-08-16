<script>
    import { goto } from '$app/navigation';
    import { fade } from 'svelte/transition';
    import { setAuth } from '$lib/io';
    import Eye from '$lib/icons/Eye.svelte';
    import Loader from '$lib/icons/Loader.svelte';

    let loading = $state(false);
    let email = $state('');
    let passwd = $state('');
    let showPasswd = $state(false);
    let isFilled = $derived(email && passwd);

    async function submit() {
        loading = true;
        const user = { email, passwd };
        try {
            const res = await setAuth(user);
            console.log(res);
            goto('/');
        } catch (err) {
            console.log(err);
            loading = false;
        }
    }
</script>

<fieldset inert={loading}>
    <legend>CrowdLogの認証情報を<wbr />入力してください。</legend>
    <label>
        <span>EMAIL</span>
        <input autofocus type="text" bind:value={email} />
    </label>
    <label style="position: relative">
        <span>PASSWORD</span>
        <input type={showPasswd ? 'text' : 'password'} bind:value={passwd} />
        <Eye class="pw-icon" hidden={!showPasswd} onclick={() => (showPasswd = !showPasswd)} />
    </label>
    <button disabled={!isFilled} onclick={submit}>OK</button>
</fieldset>

{#if loading}
    <div class="cover"><Loader></Loader></div>
{/if}

<style>
    fieldset {
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
        padding: 0.5rem 1.8rem 0.3rem 1rem;
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
        cursor: pointer;
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

    .cover {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 2rem;
        color: var(--color-text);
    }
</style>
