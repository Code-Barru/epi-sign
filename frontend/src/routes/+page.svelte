<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { currentUser } from "$lib/stores";
  import { User, Users } from "@lucide/svelte";
  import { fly, scale } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { isMobileDevice } from "$lib/utils/device";

  let isDesktop = false;

  onMount(() => {
    isDesktop = !isMobileDevice();

    // Écouter les changements de taille d'écran
    const handleResize = () => {
      isDesktop = !isMobileDevice();
    };
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  });

  function goToSelfSign() {
    if (isDesktop) return; // Désactiver sur desktop
    goto("/self-sign");
  }

  function goToDashboard() {
    goto("/dashboard");
  }
</script>

<div class="min-h-screen pb-safe">
  <div class="p-4 sm:p-6 max-w-2xl mx-auto">
    <!-- En-tête avec info utilisateur -->
    <div
      class="text-center mb-8 sm:mb-12"
      in:fly={{ y: -30, duration: 600, easing: quintOut }}
    >
      <h1 class="text-3xl sm:text-4xl font-bold gradient-text mb-4">
        Bienvenue, {$currentUser?.username || "Utilisateur"} !
      </h1>
      <p class="text-gray-400 text-base sm:text-lg">
        Choisissez votre mode de signature
      </p>
    </div>

    <!-- Options de signature -->
    <div class="space-y-4 sm:space-y-6">
      <!-- Signature personnelle -->
      <div
        in:scale={{
          duration: 500,
          delay: 200,
          easing: quintOut,
          start: 0.9,
        }}
      >
        <button
          on:click={goToSelfSign}
          disabled={isDesktop}
          class="w-full glass-effect-card rounded-2xl p-6 sm:p-8 transition-all duration-200 ease-out group
                 {isDesktop
            ? 'opacity-50 cursor-not-allowed'
            : 'hover:scale-[1.02] active:scale-[0.98] cursor-pointer'}"
        >
          <div class="flex items-center gap-4 sm:gap-6">
            <div class="flex-shrink-0">
              <div
                class="w-12 h-12 sm:w-16 sm:h-16 rounded-2xl bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center transition-transform duration-200 ease-out
                       {isDesktop ? '' : 'group-hover:scale-110'}"
              >
                <User class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
              </div>
            </div>
            <div class="flex-1 text-left">
              <h3 class="text-lg sm:text-xl font-bold text-white mb-2">
                Signer pour moi
                {#if isDesktop}
                  <span class="text-sm font-normal text-gray-500 ml-2"
                    >(Mobile uniquement)</span
                  >
                {/if}
              </h3>
              <p class="text-sm sm:text-base text-gray-400 leading-relaxed">
                Scanner un QR code pour me signer automatiquement
              </p>
              <div
                class="mt-3 flex items-center gap-2 text-xs sm:text-sm text-green-400"
              >
                <div
                  class="w-2 h-2 bg-green-400 rounded-full {isDesktop
                    ? ''
                    : 'animate-pulse'}"
                ></div>
                <span>Signature rapide et simple</span>
              </div>
            </div>
            <div class="flex-shrink-0">
              <svg
                class="w-5 h-5 sm:w-6 sm:h-6 text-gray-400 transition-all duration-200 ease-out
                       {isDesktop
                  ? ''
                  : 'group-hover:text-green-400 group-hover:translate-x-1'}"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 5l7 7-7 7"
                />
              </svg>
            </div>
          </div>
        </button>
      </div>

      <!-- Dashboard multi-utilisateurs -->
      <div
        in:scale={{
          duration: 500,
          delay: 350,
          easing: quintOut,
          start: 0.9,
        }}
      >
        <button
          on:click={goToDashboard}
          class="w-full glass-effect-card rounded-2xl p-6 sm:p-8 hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 ease-out group"
        >
          <div class="flex items-center gap-4 sm:gap-6">
            <div class="flex-shrink-0">
              <div
                class="w-12 h-12 sm:w-16 sm:h-16 rounded-2xl bg-gradient-to-br from-purple-500 to-pink-600 flex items-center justify-center group-hover:scale-110 transition-transform duration-200 ease-out"
              >
                <Users class="w-6 h-6 sm:w-8 sm:h-8 text-white" />
              </div>
            </div>
            <div class="flex-1 text-left">
              <h3 class="text-lg sm:text-xl font-bold text-white mb-2">
                Signer pour plusieurs personnes
              </h3>
              <p class="text-sm sm:text-base text-gray-400 leading-relaxed">
                Gérer et signer plusieurs utilisateurs à la fois
              </p>
              <div
                class="mt-3 flex items-center gap-2 text-xs sm:text-sm text-purple-400"
              >
                <div
                  class="w-2 h-2 bg-purple-400 rounded-full animate-pulse"
                ></div>
                <span>Gestion avancée des signatures</span>
              </div>
            </div>
            <div class="flex-shrink-0">
              <svg
                class="w-5 h-5 sm:w-6 sm:h-6 text-gray-400 group-hover:text-purple-400 group-hover:translate-x-1 transition-all duration-200 ease-out"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 5l7 7-7 7"
                />
              </svg>
            </div>
          </div>
        </button>
      </div>
    </div>

    <!-- Info utilisateur -->
    <div
      class="mt-8 sm:mt-12 text-center"
      in:fly={{ y: 30, duration: 600, delay: 500, easing: quintOut }}
    >
      <div class="glass-effect-card rounded-xl p-4 sm:p-6">
        <div class="flex items-center justify-center gap-3 mb-3">
          <div
            class="w-8 h-8 rounded-lg bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-sm font-bold"
          >
            {$currentUser?.username?.charAt(0).toUpperCase() || "?"}
          </div>
          <span class="font-medium text-gray-300">
            {$currentUser?.username || "Utilisateur"}
          </span>
        </div>

        {#if $currentUser?.jwtExpiresAt}
          {@const expiresAt = new Date($currentUser.jwtExpiresAt)}
          {@const isExpired = expiresAt < new Date()}
          <div class="flex items-center justify-center gap-2 text-sm">
            <div
              class="w-2 h-2 rounded-full {isExpired
                ? 'bg-red-400'
                : 'bg-green-400'} animate-pulse"
            ></div>
            <span class={isExpired ? "text-red-400" : "text-green-400"}>
              JWT {isExpired ? "expiré" : "valide"}
              {#if !isExpired}
                jusqu'au {expiresAt.toLocaleDateString()}
              {/if}
            </span>
          </div>
        {:else}
          <div
            class="flex items-center justify-center gap-2 text-sm text-yellow-400"
          >
            <div class="w-2 h-2 bg-yellow-400 rounded-full animate-pulse"></div>
            <span>JWT manquant - Configurez votre token</span>
          </div>
        {/if}
      </div>
    </div>

    <!-- Message informatif pour desktop -->
    {#if isDesktop}
      <div
        class="mt-6 text-center"
        in:fly={{ y: 20, duration: 400, delay: 600, easing: quintOut }}
      >
        <div
          class="glass-effect-card rounded-xl p-4 border border-blue-500/30 bg-blue-500/5"
        >
          <p class="text-sm text-blue-400">
            💡 Le mode "Signer pour moi" est optimisé pour mobile avec scanner
            QR Code
          </p>
        </div>
      </div>
    {/if}
  </div>
</div>
