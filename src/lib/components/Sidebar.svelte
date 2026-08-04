<script lang="ts">
    import { Navigation } from '@skeletonlabs/skeleton-svelte';
    import { currentPortfolioState } from '$lib/state.svelte';
    import { Sidebar, SidebarGroup, SidebarItem, SidebarBrand } from 'flowbite-svelte';
    const sidebarLinks = [
        {label: "Dashboard", href: "/"},
        {label: "Accounts", href: "/accounts"},
        {label: "Assets", href: "/"}
    ];
</script>


<Sidebar alwaysOpen class="h-full" position="absolute" backdrop = {false}>>
    <SidebarGroup>
        {#each sidebarLinks as link}
            <SidebarItem href={link.href} label={link.label} class="flex-1 ms-3 whitespace-nowrap"></SidebarItem>
        {/each}
    </SidebarGroup>
    <SidebarGroup border>
        <SidebarItem label={currentPortfolioState.id??"No portfolio"}></SidebarItem>
        <SidebarItem href="/profile" label="Profile"></SidebarItem>
    </SidebarGroup>
</Sidebar>