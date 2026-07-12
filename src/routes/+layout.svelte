<script lang="ts">
	import './layout.css';
	import Sidebar from '$lib/components/Sidebar.svelte';

	const { children } = $props();
</script>

{@render children()}

<div class="app">
	<Sidebar />

	<main>
		Hello
	</main>
</div>

<style>
	.app {
    display: grid;
    grid-template-columns: 240px 1fr;
    height: 100vh;
	}

	main {
		overflow: auto;
		padding: 1rem;
	}
</style>