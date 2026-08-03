<script lang="ts">
  import { onMount } from "svelte";
  import { Button, ButtonGroup, TableBodyRow } from "flowbite-svelte";
  import { currentPortfolioState } from "$lib/state.svelte";
  import { deleteAccount, listAccounts } from "$lib/api/account";
  import type { Account } from "$lib/api/account";
  import { Table, TableHead, TableHeadCell, TableBody, TableBodyCell } from "flowbite-svelte";

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
      <Button href = "/account/create" color="blue">Create Account</Button>
    </div>
    {#if accounts.length > 0}
        <Table shadow hoverable = {true}>
          <TableHead>
            <TableHeadCell>Name</TableHeadCell>
            <TableHeadCell>Institution</TableHeadCell>
            <TableHeadCell>Account Type</TableHeadCell>
            <TableHeadCell>Currency</TableHeadCell>
            <TableHeadCell>Created At</TableHeadCell>
            <TableHeadCell>Updated At</TableHeadCell>
            <TableHeadCell>Actions</TableHeadCell>
          </TableHead>

          <TableBody>
            {#each accounts as account}
              <TableBodyRow>
                <TableBodyCell>{account.name}</TableBodyCell>
                <TableBodyCell>{account.institution ?? "None"}</TableBodyCell>
                <TableBodyCell>{account.accountType}</TableBodyCell>
                <TableBodyCell>{account.currency}</TableBodyCell>
                <TableBodyCell>{account.createdAt}</TableBodyCell>
                <TableBodyCell>{account.updatedAt}</TableBodyCell>
                <TableBodyCell>
                  <div class="flex flex-row justify-center gap-2">
                      <Button href={`/account/${account.id}`} color="blue">Manage</Button>
                      <Button color="red" onclick={() => handleAccountDelete(account.id)}>Delete</Button>
                  </div>
                </TableBodyCell>
              </TableBodyRow>         
            {/each}
          </TableBody>
        </Table>
    {:else}
      <p class="text-center">No Accounts. Create One.</p>
    {/if}
  </section>
{:else}
  <p>No portfolio selected. Select or Create one.</p>
{/if}

