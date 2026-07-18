<script lang="ts">
    import { Navigation } from '@skeletonlabs/skeleton-svelte';
    import { currentPortfolioState } from '$lib/state.svelte';
    const sidebarLinks = [
        {label: "Dashboard", href: "/"},
        {label: "Accounts", href: "/"},
        {label: "Assets", href: "/"}
    ];
</script>

<!-- <aside>
    <div class="sidebar-title"><h2>Investment Tracker</h2></div>
    <hr>
    <nav>
        <ul>
            <li>Dashboard</li>
            <li>Accounts</li>
            <li>Assets</li>
        </ul>
    </nav>
    <hr>
    <div class="sidebar-footer"><a href="/profile">Profile</a></div>
</aside>

<style>
    aside {
        display: flex;
        flex-direction: column;
        height: 100%;
        border-right: 1px solid blanchedalmond;
    }

    nav {
        flex: 1;
    }

    nav ul {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        margin-left: 0.5rem;
    }

    hr {
        margin: 0.5rem 0;
        color: blanchedalmond;
    }

    .sidebar-title {
        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
        margin-left: 0.5rem;
    }

    .sidebar-footer {
        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
        margin-left: 0.5rem;
    }
</style> -->

<Navigation layout="sidebar" class="grid grid-rows-[auto_1fr_auto] gap-4 h-full w-full">
		<Navigation.Header>
			<a href="/">
				Investment Tracker
			</a>
		</Navigation.Header>
		<Navigation.Content>
            <Navigation.Menu>
                {#each sidebarLinks as link}
                    <Navigation.TriggerAnchor href={link.href} title={link.label} aria-label={link.label}>
                        <Navigation.TriggerText>{link.label}</Navigation.TriggerText>
                    </Navigation.TriggerAnchor>
                {/each}
            </Navigation.Menu>
		</Navigation.Content>
		<Navigation.Footer>
            <Navigation.TriggerText>
                {currentPortfolioState.id ?? "No Portfolio"}
            </Navigation.TriggerText>
			<Navigation.TriggerAnchor href="/profile" title="Profile" aria-label="Profile">
				<Navigation.TriggerText>Profile</Navigation.TriggerText>
			</Navigation.TriggerAnchor>
		</Navigation.Footer>
</Navigation>