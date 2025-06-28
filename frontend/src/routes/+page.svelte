<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { isAuthenticated } from "$lib/stores";
  import { checkAuth } from "$lib/api";

  let checking = true;

  onMount(async () => {
    await checkAuth();

    if ($isAuthenticated) {
      goto("/dashboard");
    } else {
      goto("/login");
    }

    checking = false;
  });
</script>

{#if checking}
  <div class="flex items-center justify-center min-h-screen">
    <span
      class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"
    ></span>
  </div>
{/if}
