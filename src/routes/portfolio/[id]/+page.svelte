<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import { changePortfolioBaseCurrency, getPortfolio, renamePortfolio, type Portfolio } from '$lib/api/portfolio.js';
    import { onMount } from 'svelte';

    const id = page.params.id!;

    let portfolioName = $state("");
    let baseCurrency = $state("");

    let error = $state("");

    async function handlePortfolioEdit(event:SubmitEvent) {
        event.preventDefault();

        error = "";

        try {
            await renamePortfolio({
                id: id,
                name: portfolioName,
            });

            await changePortfolioBaseCurrency({
                id: id,
                baseCurrency: baseCurrency,
            });

            await goto("/profile");
        } catch (err) {
            error = String(err);
        }


    }

    onMount(async () => {
        const portfolio = await getPortfolio(id);
        portfolioName = portfolio.name;
        baseCurrency = portfolio.baseCurrency;
    } )
</script>

<div class="card w-full preset-filled-surface-500 p-4">
  <h3 class="h3">Edit Portfolio</h3>
</div>

<section class="card mt-6 w-full p-6">
  <form onsubmit={handlePortfolioEdit}>
    <fieldset class="flex flex-col gap-2">
      <label class="label">
          <span class="label-text">Name</span> 
          <input class="input" type="text" bind:value={portfolioName} required>
      </label>

      <label class="label">
          <span class="label-text">Base Currency</span> 
          <input class="input" type="text" bind:value={baseCurrency} maxlength="3" required>
      </label>
    </fieldset>
    <fieldset class="flex justify-end gap-2 mt-4">
      <button type="submit" class="btn preset-filled-primary-500">Edit Portfolio</button>
      <a href="/profile" class="btn preset-filled-error-500">Cancel</a>
    </fieldset>
  </form>
  {#if error}
      <div class="card preset-outlined-error-500 grid grid-cols-1 items-center gap-4 p-4 lg:grid-cols-[auto_1fr_auto] mt-4">
      <div>
        <p class="font-bold">Error</p>
        <p class="text-xs opacity-60">{error}</p>
      </div>
    </div>
  {/if}
</section>