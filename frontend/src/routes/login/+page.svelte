<script lang="ts">
  import { goto } from "$app/navigation";
  import { login } from "$lib/api";
  import type { ApiError } from "$lib/types";
  import type { PageData } from "./$types";

  let username: string = "";
  let password: string = "";
  let error: string = "";
  let loading: boolean = false;

  async function handleSubmit(): Promise<void> {
    error = "";
    loading = true;

    try {
      await login(username, password);
      goto("/");
    } catch (e) {
      const apiError = e as ApiError;
      if (apiError.status === 401) {
        error = "Identifiants invalides";
      } else {
        error = "Une erreur est survenue";
      }
    } finally {
      loading = false;
    }
  }
</script>

<div class="flex items-center justify-center min-h-screen p-4 sm:p-6">
  <div class="w-full max-w-md">
    <!-- Logo mobile centré -->
    <h1
      class="text-4xl sm:text-5xl font-bold text-center mb-2 pb-2 gradient-text"
    >
      EpiSign
    </h1>
    <p class="text-center text-gray-400 mb-8 text-sm sm:text-base">
      Signatures automatisées pour l'intra d'Epitech
    </p>

    <div class="glass-effect-modal rounded-2xl p-6 sm:p-8 shadow-2xl">
      <h2 class="text-2xl font-bold text-center mb-6">Connexion</h2>

      {#if error}
        <div
          class="mb-6 p-4 bg-red-500/10 border border-red-500/50 rounded-xl text-red-400 text-sm"
        >
          {error}
        </div>
      {/if}

      <form on:submit|preventDefault={handleSubmit} class="space-y-5">
        <div>
          <label
            for="username"
            class="block text-sm font-medium text-gray-300 mb-2"
          >
            Nom d'utilisateur
          </label>
          <input
            type="text"
            id="username"
            bind:value={username}
            required
            autocomplete="username"
            class="input-field"
            placeholder="Votre identifiant"
          />
        </div>

        <div>
          <label
            for="password"
            class="block text-sm font-medium text-gray-300 mb-2"
          >
            Mot de passe
          </label>
          <input
            type="password"
            id="password"
            bind:value={password}
            required
            autocomplete="current-password"
            class="input-field"
            placeholder="Votre mot de passe"
          />
        </div>

        <button type="submit" disabled={loading} class="btn-primary mt-6">
          {#if loading}
            <span
              class="inline-block animate-spin rounded-full h-5 w-5 border-b-2 border-white"
            ></span>
          {:else}
            Se connecter
          {/if}
        </button>
      </form>

      <div class="mt-6 text-center">
        <a
          href="/register"
          class="text-purple-400 active:text-purple-300 font-medium transition-colors"
        >
          Créer un compte
        </a>
      </div>
    </div>
  </div>
</div>
