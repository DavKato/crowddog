<script>
	import { goto } from '$app/navigation';
	import { store } from '$lib';

	if (!store.user) goto('/login', { replaceState: true });
</script>
