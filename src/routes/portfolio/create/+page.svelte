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

<h1>Create Portfolio</h1>

<p>Create a new investment portfolio.</p>
<section>
<form onsubmit={handlePortfolioSubmit}>
    <label>
        Name
        <input bind:value={portfolioName} required>
    </label>

    <label>
        Base Currency
        <input bind:value={baseCurrency} maxlength="3">
    </label>

    <button type="submit">Create Portfolio</button>
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