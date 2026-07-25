<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import { AccountType, activateAccount, changeAccountCurrency, changeAccountInstitution, changeAccountType, deactivateAccount, renameAccount } from '$lib/api/account';
    import { getAccount } from '$lib/api/account';

    const id = page.params.id!;

    let accountName = $state("");
    let currency = $state("");
    let institution = $state("") as string|null;
    let accountType = $state<AccountType>(AccountType.Brokerage);
    let isActive = $state(false);

    let error = $state("");

    async function handleAccountEdit(event:SubmitEvent) {
        event.preventDefault();

        error = "";

        try {
            await renameAccount({
                id: id,
                name: accountName,
            });

            await changeAccountCurrency({
                id: id,
                currency: currency,
            });

            await changeAccountInstitution({
                id: id,
                institution: institution,
            });

            await changeAccountType({
                id: id,
                accountType: accountType,
            })

            await goto("/accounts");
        } catch (err) {
            error = String(err);
        }


    }

    async function handleAccountActivationToggle() {
        error = "";
        try {
            if (isActive) {
                await deactivateAccount(id);
                isActive = false;
            } else {
                await activateAccount(id);
                isActive = true;
            }
        } catch (err) {
            error = String(err);
        }
    }

    onMount(async () => {
        const account = await getAccount(id);

        accountName = account.name;
        currency = account.currency;
        institution = account.institution;
        accountType = account.accountType;
        isActive = account.isActive;
    } )
</script>

<div class="card w-full preset-filled-surface-500 p-4">
  <h3 class="h3">Edit Account</h3>
</div>

<section class="card mt-6 w-full p-6">
  <form onsubmit={handleAccountEdit}>
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
    <fieldset class="flex justify-between mt-4">
      <button type="button" class="btn preset-filled-secondary-500" onclick={handleAccountActivationToggle}>{isActive? "Deactivate" : "Activate"}</button>
      <div class="flex gap-2">
      <button type="submit" class="btn preset-filled-primary-500">Edit Account</button>
      <a href="/accounts" class="btn preset-filled-error-500">Cancel</a>
      </div>
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