<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import UserCard from "$lib/components/UserCard.svelte";
  import type { PublicUserResponse } from "$lib/types";

  export let users: PublicUserResponse[];
  export let selectedUsers: Set<string>;
  export let loading: boolean;

  const dispatch = createEventDispatcher();

  function handleUserToggle(event: CustomEvent<string>) {
    dispatch("userToggle", event.detail);
  }
</script>

{#if loading}
  <div class="flex justify-center py-12">
    <span
      class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"
    ></span>
  </div>
{:else if users.length === 0}
  <div class="text-center py-12 text-gray-400">Aucun utilisateur trouvé</div>
{:else}
  <div class="space-y-3">
    {#each users as user}
      <UserCard
        {user}
        isSelected={selectedUsers.has(user.id)}
        on:toggle={handleUserToggle}
      />
    {/each}
  </div>
{/if}
