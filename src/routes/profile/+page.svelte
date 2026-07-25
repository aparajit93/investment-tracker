<script lang="ts">
  import { onMount } from "svelte";
  import { createPortfolio, listPortfolios, renamePortfolio, deletePortfolio, changePortfolioBaseCurrency } from "$lib/api/portfolio";
  import type { Portfolio } from "$lib/api/portfolio";
  import { goto } from "$app/navigation";
  import { currentPortfolioState } from "$lib/state.svelte";

  let portfolios = $state<Portfolio[]>([]);

  let portfolioName = $state("");
  let baseCurrency = $state("USD");

  let error = $state("");


  async function loadPortfolios() {
    portfolios = await listPortfolios();

    if (portfolios.length === 0) {
      currentPortfolioState.id = null;
      return
    }

    if (currentPortfolioState.id === null || !portfolios.some(portfolio => portfolio.id === currentPortfolioState.id)) {
      currentPortfolioState.id = portfolios[0].id;
    }
  }

  async function handlePortfolioSubmit(event:SubmitEvent) {
    event.preventDefault();

    error = "";

    try {
        await createPortfolio({
            name: portfolioName,
            baseCurrency,
        });
        portfolioName = "";
        await loadPortfolios();
    } catch (err) {
        error = String(err);
    }
  }

  async function handleDeletePortfolio(id: string) {
    if (portfolios.length == 0) {
      return;
    }

    await deletePortfolio(id);

    await loadPortfolios();
  }

  async function handleSwitchActivePortfolio(id:string) {
    currentPortfolioState.id = id;
  }

  onMount(loadPortfolios);
</script>

<div class="card w-full preset-filled-surface-500 p-4">
  <h1 class="h1 text-center">Profile</h1>
</div>
<section>
  <div class="mt-8 mb-4 flex flex-row justify-between">
    <h3 class="h3">Portfolios</h3>
    <a href="/portfolio/create" class="btn preset-filled-primary-500">Create Portfolio</a>
    <!-- <button type="button" class="btn preset-filled-primary-500" onclick={() => goto("/portfolio/create")}>Create Portfolio</button> -->
  </div>
  <div class="table-wrap">
    <table class="table table-auto">

      <thead>
        <tr>
          <th>Name</th>
          <th>Base Currency</th>
          <th>Created At</th>
          <th>Updated At</th>
          <th class="text-center!">Actions</th>
        </tr>
      </thead>

      <tbody class="[&>tr]:hover:preset-tonal-primary">
        {#each portfolios as portfolio}
          <tr>
            <td>{portfolio.name}</td>
            <td>{portfolio.baseCurrency}</td>
            <td>{portfolio.createdAt}</td>
            <td>{portfolio.updatedAt}</td>
            <td>
              <div class="flex flex-row justify-center gap-2">
              <a href={`/portfolio/${portfolio.id}`} class="btn preset-filled-primary-500">Manage</a>
              <!-- <button type="button" class="btn preset-filled-primary-500" onclick={() => goto(`/portfolio/${portfolio.id}`)}>Manage</button> -->
              <button type="button" class="btn preset-filled-secondary-500" onclick={() => handleSwitchActivePortfolio(portfolio.id)}>Set Active</button>
              <button type="button" class="btn preset-filled-error-500" onclick={() => handleDeletePortfolio(portfolio.id)}>Delete</button>
              </div>
            </td>
          </tr>
          
        {/each}
      </tbody>
    </table>
  </div>
</section>

