<script lang="ts">
    import { createAccount } from "$lib/api/account";
    import { AccountType } from "$lib/api/account";
    import { currentPortfolioState } from "$lib/state.svelte";
    import { goto } from "$app/navigation";

    let accountName = $state("");
    let currency = $state("USD");
    let institution = $state("");
    let accountType = $state<AccountType>(AccountType.Brokerage);

    let error = $state("");

    async function handleAccountSubmit(event:SubmitEvent) {
        event.preventDefault();

        error = "";

        if (!currentPortfolioState.id) {
            error = "No portfolio selected.";
            return
        }

        try {
            await createAccount({
                portfolioId: currentPortfolioState.id,
                name: accountName,
                institution: institution.trim() || null,
                accountType: accountType,
                currency: currency
            });
            await goto("/accounts")
        } catch (err) {
            error = String(err);
        }
        
    }
</script>

<div class="card w-full preset-filled-surface-500 p-4">
  <h3 class="h3">Create Account</h3>
</div>

<section class="card mt-6 w-full p-6">
  <form onsubmit={handleAccountSubmit}>
    <fieldset class="flex flex-col gap-2">
      <label class="label">
          <span class="label-text">Name</span> 
          <input class="input" type="text" bind:value={accountName} required>
      </label>

      <label class="label">
          <span class="label-text">Currency</span> 
          <input class="input" type="text" bind:value={currency} maxlength="3" required>
      </label>
    </fieldset>
    <fieldset class="flex flex-col gap-2">
      <label class="label">
          <span class="label-text">Institution</span> 
          <input class="input" type="text" bind:value={institution}>
      </label>

      <label class="label">
          <span class="label-text">Account Type</span> 
          <select class="select" bind:value={accountType} required>
            {#each Object.values(AccountType) as type}
                <option value={type}>{type}</option>
            {/each}
          </select>
      </label>
    </fieldset>
    <fieldset class="flex justify-end gap-2 mt-4">
      <button type="submit" class="btn preset-filled-primary-500">Create Account</button>
      <a href="/accounts" class="btn preset-filled-error-500">Cancel</a>
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