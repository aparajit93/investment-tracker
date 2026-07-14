<script lang="ts">
  import { onMount } from "svelte";
  import { createPortfolio, listPortfolios, renamePortfolio, deletePortfolio, changePortfolioBaseCurrency } from "$lib/api/portfolio";
  import type { Portfolio } from "$lib/api/portfolio";

  let portfolios = $state<Portfolio[]>([]);

  let portfolioName = $state("");
  let baseCurrency = $state("USD");

  let error = $state("");


  async function loadPortfolios() {
    portfolios = await listPortfolios();
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

  onMount(loadPortfolios);
</script>

<main class="container">

  <h1>Profile</h1>
  <h2>Manage Portfolios</h2>
  <section>
    <a href="/portfolio/create">Create Portfolio</a>
  </section>
  
  <section>
    <h2>Portfolios</h2>
    <table>

      <thead>
        <tr>
          <th>Name</th>
          <th>Base Currency</th>
          <th>Created At</th>
          <th>Actions</th>
        </tr>
      </thead>

      <tbody>
        {#each portfolios as portfolio}
          <tr>
            <td>{portfolio.name}</td>
            <td>{portfolio.baseCurrency}</td>
            <td>{portfolio.createdAt}</td>
            <td>
              <a href={`/portfolio/${portfolio.id}`}>Manage</a>
              <button onclick={() => handleDeletePortfolio(portfolio.id)}>Delete</button>
            </td>
          </tr>
          
        {/each}
      </tbody>
    </table>
  </section>

</main>

<style>
a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}
</style>