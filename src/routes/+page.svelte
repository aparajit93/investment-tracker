<script lang="ts">
    import { onMount } from "svelte";
    import { currentPortfolioState } from "$lib/state.svelte";
    import { listPortfolios } from "$lib/api/portfolio";

    onMount(async () => {
        const portfolios = await listPortfolios();

        if (portfolios.length > 0 && currentPortfolioState.id === null) {
            currentPortfolioState.id = portfolios[0].id;
        }
    });
</script>