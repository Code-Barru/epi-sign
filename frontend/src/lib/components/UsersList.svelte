<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import UserCard from "$lib/components/UserCard.svelte";
  import type { PublicUserResponse } from "$lib/types";
  import { currentUser } from "$lib/stores";
  import { fly, fade } from "svelte/transition";
  import { quintOut } from "svelte/easing";

  export let users: PublicUserResponse[];
  export let selectedUsers: Set<string>;
  export let loading: boolean;

  const dispatch = createEventDispatcher();

  function handleUserToggle(event: CustomEvent<string>) {
    dispatch("userToggle", event.detail);
  }

  // Séparer l'utilisateur actuel des autres
  $: currentUserData = users.find((user) => user.id === $currentUser?.id);
  $: otherUsers = users.filter((user) => user.id !== $currentUser?.id);
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
    <!-- Utilisateur actuel -->
    {#if currentUserData}
      <div in:fly={{ x: -20, duration: 300, easing: quintOut }}>
        <UserCard
          user={currentUserData}
          isSelected={selectedUsers.has(currentUserData.id)}
          on:toggle={handleUserToggle}
        />
      </div>

      <!-- Séparateur néon simple -->
      {#if otherUsers.length > 0}
        <div
          class="my-4 flex justify-center items-center"
          in:fade={{ delay: 200, duration: 400 }}
        >
          <div
            class="h-1 w-7/8 bg-gradient-to-r from-purple-500 via-pink-500 to-purple-500 rounded-full shadow-lg shadow-purple-500/50 animate-pulse"
          ></div>
        </div>
      {/if}
    {/if}

    <!-- Autres utilisateurs -->
    {#each otherUsers as user, index}
      <div
        in:fly={{
          x: -20,
          duration: 300,
          delay: currentUserData ? 100 + index * 50 : index * 50,
          easing: quintOut,
        }}
      >
        <UserCard
          {user}
          isSelected={selectedUsers.has(user.id)}
          on:toggle={handleUserToggle}
        />
      </div>
    {/each}
  </div>
{/if}
