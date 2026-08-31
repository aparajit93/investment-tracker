<script lang="ts">
    import AppSidebar from '$lib/components/AppSidebar.svelte';
    import * as Sidebar from '$lib/components/ui/sidebar/index';
	import './layout.css';

	const { children } = $props();
</script>

<!-- {@render children()} -->
<!-- <div class="app w-full h-screen grid grid-cols-[auto_1fr] items-stretch border border-surface-200-800">
	<Sidebar />
	<main>{@render children()}</main>
</div> -->

<Sidebar.Provider>
	<AppSidebar />
		<main>{@render children()}</main>
</Sidebar.Provider>

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
