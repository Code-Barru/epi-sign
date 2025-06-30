<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { updateUserJWT } from "$lib/api";
  import type { ApiError } from "$lib/types";
  import { fly, fade, scale } from "svelte/transition";
  import { quintOut } from "svelte/easing";

  export let isOpen: boolean = false;

  const dispatch = createEventDispatcher();

  let jwt: string = "";
  let loading: boolean = false;
  let error: string = "";
  let success: boolean = false;
  let showPassword: boolean = false;

  async function handleSubmit() {
    error = "";
    success = false;

    if (!jwt.trim()) {
      error = "Veuillez entrer un JWT";
      return;
    }

    // Validation basique du format JWT
    const jwtParts = jwt.split(".");
    if (jwtParts.length !== 3) {
      error =
        "Format JWT invalide (doit contenir 3 parties séparées par des points)";
      return;
    }

    loading = true;

    try {
      await updateUserJWT(jwt);
      success = true;

      // Fermer après 2 secondes de succès
      setTimeout(() => {
        handleClose();
      }, 2000);
    } catch (e) {
      const apiError = e as ApiError;
      if (apiError.status === 400) {
        error = "JWT invalide ou mal formaté";
      } else if (apiError.status === 401) {
        error = "Non autorisé - Veuillez vous reconnecter";
      } else {
        error = "Erreur lors de la mise à jour du JWT";
      }
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    jwt = "";
    error = "";
    success = false;
    showPassword = false;
    dispatch("close");
  }

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  // Fermer avec Escape
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && isOpen) {
      handleClose();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    in:fade={{ duration: 300, easing: quintOut }}
    out:fade={{ duration: 200, easing: quintOut }}
  >
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-black/50 backdrop-blur-sm"
      on:click={handleClose}
      in:fade={{ duration: 300 }}
      out:fade={{ duration: 200 }}
      aria-label="Fermer le modal"
    ></button>

    <!-- Modal -->
    <div
      class="relative w-full max-w-lg glass-effect rounded-2xl p-6 sm:p-8 shadow-2xl"
      in:scale={{
        duration: 400,
        easing: quintOut,
        start: 0.9,
        opacity: 0,
      }}
      out:scale={{
        duration: 200,
        easing: quintOut,
        start: 0.95,
        opacity: 0,
      }}
    >
      <!-- Header -->
      <div
        class="flex justify-between items-start mb-6"
        in:fly={{ y: -20, duration: 400, delay: 100, easing: quintOut }}
      >
        <div>
          <h2 class="text-2xl font-bold gradient-text">Mettre à jour le JWT</h2>
          <p class="text-sm text-gray-400 mt-2">
            Entrez votre JWT Intranet Epitech
          </p>
        </div>
        <button
          on:click={handleClose}
          class="p-2 rounded-lg hover:bg-white/10 transition-all duration-200 ease-out transform hover:scale-110 active:scale-95 hover:rotate-90"
          aria-label="Fermer"
        >
          <svg
            class="w-6 h-6"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <!-- Alerts -->
      {#if error}
        <div
          class="mb-6 p-4 bg-red-500/10 border border-red-500/50 rounded-xl text-red-400 text-sm"
          in:fly={{ x: -50, duration: 300, easing: quintOut }}
          out:fly={{ x: 50, duration: 200, easing: quintOut }}
        >
          ❌ {error}
        </div>
      {/if}

      {#if success}
        <div
          class="mb-6 p-4 bg-green-500/10 border border-green-500/50 rounded-xl text-green-400 text-sm"
          in:scale={{ duration: 400, easing: quintOut }}
          out:scale={{ duration: 200, easing: quintOut }}
        >
          ✅ JWT mis à jour avec succès !
        </div>
      {/if}

      <!-- Form -->
      <form
        on:submit|preventDefault={handleSubmit}
        class="space-y-6"
        in:fly={{ y: 20, duration: 400, delay: 200, easing: quintOut }}
      >
        <div
          class="transform transition-all duration-200 ease-out hover:scale-[1.02]"
        >
          <label for="jwt" class="block text-sm font-medium text-gray-300 mb-2">
            Token JWT
          </label>
          <div class="relative">
            <textarea
              id="jwt"
              bind:value={jwt}
              rows="4"
              disabled={loading || success}
              placeholder="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
              class="input-field resize-none font-mono text-sm pr-12 transition-all duration-200 ease-out focus:scale-[1.02]"
              style="word-break: break-all; {showPassword
                ? ''
                : '-webkit-text-security: disc; text-security: disc;'}"
            ></textarea>
            <button
              type="button"
              on:click={togglePasswordVisibility}
              class="absolute right-3 top-3 p-2 rounded-lg hover:bg-white/10 transition-all duration-200 ease-out transform hover:scale-110 active:scale-95"
              aria-label={showPassword ? "Masquer" : "Afficher"}
            >
              {#if showPassword}
                <svg
                  class="w-5 h-5 text-gray-400 transition-transform duration-300 ease-out"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  in:scale={{ duration: 200 }}
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
                  />
                </svg>
              {:else}
                <svg
                  class="w-5 h-5 text-gray-400 transition-transform duration-300 ease-out"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  in:scale={{ duration: 200 }}
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                  />
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                  />
                </svg>
              {/if}
            </button>
          </div>
          <p class="mt-2 text-xs text-gray-500">
            Le JWT doit contenir 3 parties séparées par des points
          </p>
        </div>

        <!-- Actions -->
        <div
          class="flex gap-3"
          in:fly={{ y: 20, duration: 300, delay: 300, easing: quintOut }}
        >
          <button
            type="button"
            on:click={handleClose}
            disabled={loading}
            class="btn-secondary flex-1 transform transition-all duration-200 ease-out hover:scale-105 active:scale-95"
          >
            Annuler
          </button>
          <button
            type="submit"
            disabled={loading || success}
            class="btn-primary flex-1 transform transition-all duration-200 ease-out hover:scale-105 active:scale-95"
          >
            {#if loading}
              <span
                class="inline-block animate-spin rounded-full h-5 w-5 border-b-2 border-white"
                in:scale={{ duration: 300 }}
              ></span>
            {:else if success}
              <span in:scale={{ duration: 300, easing: quintOut }}
                >Mis à jour !</span
              >
            {:else}
              Mettre à jour
            {/if}
          </button>
        </div>
      </form>

      <!-- Help -->
      <div class="mt-6 text-center" in:fade={{ delay: 400, duration: 300 }}>
        <details
          class="text-sm text-gray-400 transform transition-all duration-200 ease-out hover:scale-[1.02]"
        >
          <summary
            class="cursor-pointer hover:text-gray-300 transition-colors duration-200 ease-out"
          >
            Comment obtenir mon JWT ?
          </summary>
          <div
            class="mt-3 text-left space-y-2 bg-white/5 rounded-xl p-4"
            in:fly={{ y: 10, duration: 300, easing: quintOut }}
          >
            <p class="font-semibold">Sur Chrome/Edge :</p>
            <ol class="space-y-1 text-xs">
              <li
                class="transform transition-all duration-200 ease-out hover:scale-[1.02]"
              >
                1. Connectez-vous à l'Intranet Epitech
              </li>
              <li
                class="transform transition-all duration-200 ease-out hover:scale-[1.02]"
              >
                2. Ouvrez les DevTools (F12)
              </li>
              <li
                class="transform transition-all duration-200 ease-out hover:scale-[1.02]"
              >
                3. Allez dans l'onglet "Application" ou "Storage"
              </li>
              <li
                class="transform transition-all duration-200 ease-out hover:scale-[1.02]"
              >
                4. Dans "Cookies" → trouvez "user"
              </li>
              <li
                class="transform transition-all duration-200 ease-out hover:scale-[1.02]"
              >
                5. Copiez la valeur complète
              </li>
            </ol>
          </div>
        </details>
      </div>
    </div>
  </div>
{/if}
