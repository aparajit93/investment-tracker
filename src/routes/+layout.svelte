<script lang="ts">
	import './layout.css';
	import Sidebar from '$lib/components/Sidebar.svelte';

	const { children } = $props();
</script>

<!-- {@render children()} -->
<div class="app w-full h-screen grid grid-cols-[auto_1fr] items-stretch border border-surface-200-800">
	<Sidebar />
	<main>{@render children()}</main>
</div>

<style>
	/* .app {
		display: grid;
		grid-template-columns: 240px 1fr;
		height: 100vh;
	} */

	main {
		overflow: auto;
		padding: 1rem;
	}
</style>
