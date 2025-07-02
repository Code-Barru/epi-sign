<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { Home, ArrowLeft } from "@lucide/svelte";
  import { isAuthenticated } from "$lib/stores";

  function goHome() {
    if ($isAuthenticated) {
      goto("/");
    } else {
      goto("/login");
    }
  }

  function goBack() {
    history.back();
  }
</script>

<div class="min-h-screen flex items-center justify-center p-4">
  <div class="text-center max-w-md">
    <!-- Logo -->
    <h1 class="text-4xl font-bold gradient-text mb-6">EpiSign</h1>

    <!-- Erreur 404 -->
    <div class="glass-effect-modal rounded-2xl p-8 mb-6">
      <div class="text-6xl mb-4">🤔</div>
      <h2 class="text-2xl font-bold text-white mb-3">Page introuvable</h2>
      <p class="text-gray-400 mb-6">
        La page que vous cherchez n'existe pas ou a été déplacée.
      </p>

      <!-- Boutons d'action -->
      <div class="flex gap-3">
        <button
          on:click={goBack}
          class="btn-secondary flex-1 flex items-center justify-center gap-2"
        >
          <ArrowLeft class="w-4 h-4" />
          Retour
        </button>

        <button
          on:click={goHome}
          class="btn-primary flex-1 flex items-center justify-center gap-2"
        >
          <Home class="w-4 h-4" />
          Accueil
        </button>
      </div>
    </div>

    <!-- Info debug en développement -->
    {#if $page.url.pathname}
      <p class="text-xs text-gray-500 font-mono">
        Route demandée : {$page.url.pathname}
      </p>
    {/if}
  </div>
</div>
