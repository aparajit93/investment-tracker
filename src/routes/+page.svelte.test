<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { createPortfolio, listPortfolios, renamePortfolio, deletePortfolio, changePortfolioBaseCurrency } from "../lib/api/portfolio";
  import type { Portfolio } from "../lib/api/portfolio";
  import {
    AccountType, 
    createAccount, 
    listAccounts, 
    renameAccount, 
    changeAccountCurrency, 
    changeAccountInstitution, 
    changeAccountType, 
    deactivateAccount,
    activateAccount,
    deleteAccount} from "$lib/api/account";
  import type { Account } from "$lib/api/account";

  // let name = $state("");
  // let greetMsg = $state("");

  let portfolios = $state<Portfolio[]>([]);
  let accounts = $state<Account[]>([]);

  let portfolioName = $state("");
  let baseCurrency = $state("USD");

  let error = $state("");

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }

  async function loadPortfolios() {
    portfolios = await listPortfolios();
  }

  async function mountLoad() {
    portfolios = await listPortfolios();
    accounts = await listAccounts(portfolios[0].id);
  }
  async function loadAccounts() {
    accounts = await listAccounts(portfolios[0].id);
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

  // ACCOUNT TESTING

    async function handleAccountSubmit() {
    // event.preventDefault();

    // error = "";

    try {
        await createAccount({
          portfolioId: portfolios[0].id, 
          name: "TEST", 
          institution: "TRIAL", 
          accountType: AccountType.Brokerage, 
          currency: "USD"
        });
        await loadAccounts();
    } catch (err) {
        error = String(err);
    }
  }

  async function renameFirstAccount() {
    if (accounts.length == 0) {
      return;
    }

    const first = accounts[0];

    await renameAccount({id: first.id, name: `${first.name}_updated`});

    await loadAccounts();
  }

  async function changeFirstAccountCurrency() {
    if (accounts.length == 0) {
      return;
    }

    const first = accounts[0];

    await changeAccountCurrency({id: first.id, currency: "CAD"});

    await loadAccounts();
  }

  async function changeFirstAccountInstitution() {
    if (accounts.length == 0) {
      return;
    }

    const first = accounts[0];

    await changeAccountInstitution({id: first.id, institution: "BANK"});

    await loadAccounts();
  }

  async function changeFirstAccountType() {
    if (accounts.length == 0) {
      return;
    }

    const first = accounts[0];

    await changeAccountType({id: first.id, accountType: AccountType.Savings});

    await loadAccounts();
  }

  async function deactivateFirstAccount() {
    if (accounts.length == 0) {
      return;
    }

    const first = accounts[0];

    await deactivateAccount(first.id);

    await loadAccounts();
  }

  async function activateFirstAccount() {
    if (accounts.length == 0) {
      return;
    }

    const first = accounts[0];

    await activateAccount(first.id);

    await loadAccounts();
  }

  async function deleteFirstAccount() {
    if (portfolios.length == 0) {
      return;
    }

    await deleteAccount(accounts[0].id);

    await loadAccounts();
  }

  // onMount(loadPortfolios);
  onMount(mountLoad);
</script>

<main class="container">
  <!-- <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p> -->

  <h1>PORTFOLIO MANAGEMENT</h1>
  <section>
    <h2> Create Portfolio </h2>
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
  </form>
  {#if error}
    <p>{error}</p>
  {/if}
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
              <button>Edit</button>
              <button onclick={() => handleDeletePortfolio(portfolio.id)}>Delete</button>
            </td>
          </tr>
          
        {/each}
      </tbody>
    </table>
  </section>

  <h1>ACCOUNT MANAGEMENT</h1>
  <button onclick={handleAccountSubmit}>Create Acccount</button>
  <ul>
    {#each accounts as account}
      <li>{account.name}</li>
      <li>{account.institution}</li>
      <li>{account.accountType}</li>
      <li>{account.currency}</li>
      <li>{account.isActive}</li>
    {/each}
  </ul>

  <button onclick={renameFirstAccount}>
    Rename First Account
  </button>

  <button onclick={changeFirstAccountCurrency}>
    Change First Account Currency
  </button>

  <button onclick={changeFirstAccountInstitution}>
    Change First Account Institution
  </button>

  <button onclick={changeFirstAccountType}>
    Change First Account Type
  </button>

  <button onclick={deactivateFirstAccount}>
    Deactivate First Account
  </button>

  <button onclick={activateFirstAccount}>
    Activate First Account
  </button>

  <button onclick={deleteFirstAccount}>
    Delete First Account
  </button>
</main>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

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

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
