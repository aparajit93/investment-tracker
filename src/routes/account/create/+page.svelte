<script lang="ts">
    import { createAccount } from "$lib/api/account";
    import { AccountType } from "$lib/api/account";
    import { currentPortfolioState } from "$lib/state.svelte";
    import { goto } from "$app/navigation";
    import { Label, Input, Select, Alert } from "flowbite-svelte";

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
    <div class="flex flex-col gap-2">
      <div>
          <Label>Name</Label> 
          <Input type="text" bind:value={accountName} required />
      </div>

      <div>
          <Label>Currency</Label> 
          <Input type="text" bind:value={currency} required maxlength = {3}/>
      </div>
    </div>
    <fieldset class="flex flex-col gap-2">
      <label class="label">
          <span class="label-text">Institution</span> 
          <input class="input" type="text" bind:value={institution}>
      </label>

      <label class="label">
          <span class="label-text">Account Type</span> 
          <!-- <select class="select" bind:value={accountType} required>
            {#each Object.values(AccountType) as type}
                <option value={type}>{type}</option>
            {/each}
          </select> -->
          <Select bind:value={accountType} required>
            {#each Object.values(AccountType) as type}
                <option value={type}>{type}</option>
            {/each}
          </Select>
      </label>
    </fieldset>
    <fieldset class="flex justify-end gap-2 mt-4">
      <button type="submit" class="btn preset-filled-primary-500">Create Account</button>
      <a href="/accounts" class="btn preset-filled-error-500">Cancel</a>
    </fieldset>
  </form>
  {#if error}
      <Alert color="red" class="mt-2">
        <span class="font-medium">Error:</span>
        {error}
      </Alert>
  {/if}
</section>