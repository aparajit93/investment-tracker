<script lang="ts">
    import { createPortfolio } from "$lib/api/portfolio";
    import type { Portfolio } from "$lib/api/portfolio";
    import { goto } from "$app/navigation";

    let portfolioName = $state("");
    let baseCurrency = $state("USD");

    let error = $state("");

    async function handlePortfolioSubmit(event:SubmitEvent) {
        event.preventDefault();

        error = "";

        try {
            await createPortfolio({
                name: portfolioName,
                baseCurrency,
            });
            await goto("/profile")
        } catch (err) {
            error = String(err);
        }
    }
</script>

<div class="card w-full preset-filled-surface-500 p-4">
  <h3 class="h3">Create Portfolio</h3>
</div>

<section class="card mt-6 w-full p-6">
  <form onsubmit={handlePortfolioSubmit}>
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
      <button type="submit" class="btn preset-filled-primary-500">Create Portfolio</button>
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