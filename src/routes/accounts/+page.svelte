<script lang="ts">
  import { onMount } from "svelte";
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { currentPortfolioState } from "$lib/state.svelte";
  import { deleteAccount, listAccounts } from "$lib/api/account";
  import type { Account } from "$lib/api/account";

  let accounts = $state<Account[]>([]);
  let error = $state("");


  async function loadAccounts(id: string|null) {
    if (!id) {
        accounts = [];
        return
    }

    accounts = await listAccounts(id);
  }

  async function handleAccountDelete(id: string) {
    if (accounts.length == 0) {
      return;
    }

    await deleteAccount(id);

    await loadAccounts(currentPortfolioState.id);
  }

  $effect(() => {
    loadAccounts(currentPortfolioState.id);
  });

  onMount(async () => {
    await loadAccounts(currentPortfolioState.id);
  });
</script>

<div class="card w-full preset-filled-surface-500 p-4">
  <h1 class="h1 text-center">Accounts</h1>
</div>
{#if currentPortfolioState.id}
  <section>
    <div class="mt-8 mb-4 flex flex-row justify-between">
      <h3 class="h3">Accounts</h3>
      <!-- <a href="/account/create" class="btn preset-filled-primary-500">Create Account</a> -->
      <Button href = "/account/create" color="blue">Create Account</Button>
    </div>
    {#if accounts.length > 0}
      <div class="table-wrap">
        <table class="table table-auto">

          <thead>
            <tr>
              <th>Name</th>
              <th>Institution</th>
              <th>Account Type</th>
              <th>Currency</th>
              <th>Created At</th>
              <th>Updated At</th>
              <th>Actions</th>
            </tr>
          </thead>

          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each accounts as account}
              <tr>
                <td>{account.name}</td>
                <td>{account.institution ?? "None"}</td>
                <td>{account.accountType}</td>
                <td>{account.currency}</td>
                <td>{account.createdAt}</td>
                <td>{account.updatedAt}</td>
                <td>
                  <div class="flex flex-row justify-center gap-2">
                    <!-- <a href={`/account/${account.id}`} class="btn preset-filled-primary-500">Manage</a>
                    <button type="button" class="btn preset-filled-error-500" onclick={() => handleAccountDelete(account.id)}>Delete</button> -->
                    <!-- <ButtonGroup> -->
                      <Button href={`/account/${account.id}`} color="blue">Manage</Button>
                      <Button color="red" onclick={() => handleAccountDelete(account.id)}>Delete</Button>
                    <!-- </ButtonGroup> -->
                  </div>
                </td>
              </tr>         
            {/each}
          </tbody>
        </table>
      </div>
    {:else}
      <p class="text-center">No Accounts. Create One.</p>
    {/if}
  </section>
{:else}
  <p>No portfolio selected. Select or Create one.</p>
{/if}

