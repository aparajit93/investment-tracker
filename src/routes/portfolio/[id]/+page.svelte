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

<h1>Edit Portfolio</h1>

<p>Edit investment portfolio</p>
<section>
<form onsubmit={handlePortfolioEdit}>
    <label>
        Name
        <input bind:value={portfolioName} required>
    </label>

    <label>
        Base Currency
        <input bind:value={baseCurrency} maxlength="3">
    </label>

    <button type="submit">Edit Portfolio</button>
    <a href="/profile">Cancel</a>
</form>
{#if error}
    <p>{error}</p>
{/if}
</section>

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