<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { currentUser } from "$lib/stores";
  import { logout } from "$lib/api";
  import { goto } from "$app/navigation";
  import { clickOutside } from "$lib/actions/clickOutside";

  export let isMobile: boolean = false;

  const dispatch = createEventDispatcher();

  let isOpen: boolean = false;
  let jwtStatus: "valid" | "expired" | "missing" = "missing";

  // Vérifier le statut du JWT
  $: {
    if ($currentUser?.jwtExpiresAt) {
      const expiresAt = new Date($currentUser.jwtExpiresAt);
      const now = new Date();
      jwtStatus = expiresAt > now ? "valid" : "expired";
    } else {
      jwtStatus = "missing";
    }
  }

  function toggleMenu() {
    isOpen = !isOpen;
  }

  function closeMenu() {
    isOpen = false;
  }

  async function handleLogout() {
    closeMenu();
    await logout();
    goto("/login");
  }

  function handleUpdateJWT() {
    closeMenu();
    dispatch("updateJWT");
  }

  function getJWTStatusConfig() {
    switch (jwtStatus) {
      case "valid":
        return {
          color: "text-green-400 bg-green-500/10 border-green-500/30",
          icon: "✓",
          text: "JWT valide",
          needsUpdate: false,
        };
      case "expired":
        return {
          color: "text-red-400 bg-red-500/10 border-red-500/30",
          icon: "!",
          text: "JWT expiré",
          needsUpdate: true,
        };
      case "missing":
        return {
          color: "text-yellow-400 bg-yellow-500/10 border-yellow-500/30",
          icon: "?",
          text: "JWT manquant",
          needsUpdate: true,
        };
    }
  }

  $: statusConfig = getJWTStatusConfig();
</script>

<div class="relative" use:clickOutside={closeMenu}>
  <!-- Trigger Button -->
  <button
    on:click={toggleMenu}
    class="flex items-center gap-2 p-2 rounded-xl hover:bg-white/10 transition-colors"
    aria-expanded={isOpen}
    aria-haspopup="menu"
  >
    <!-- Avatar/Initial -->
    <div
      class="w-8 h-8 rounded-lg bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-sm font-bold"
    >
      {$currentUser?.username?.charAt(0).toUpperCase() || "?"}
    </div>

    <!-- Username (desktop only) -->
    {#if !isMobile}
      <span class="text-sm font-medium"
        >{$currentUser?.username || "Utilisateur"}</span
      >
    {/if}

    <!-- JWT Status Indicator -->
    <div class="relative">
      <div
        class="w-2 h-2 rounded-full {statusConfig.needsUpdate
          ? 'bg-red-500'
          : 'bg-green-500'} animate-pulse"
      ></div>
    </div>

    <!-- Chevron -->
    <svg
      class="w-4 h-4 transition-transform {isOpen ? 'rotate-180' : ''}"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M19 9l-7 7-7-7"
      />
    </svg>
  </button>

  <!-- Dropdown Menu -->
  {#if isOpen}
    <div
      class="absolute right-0 mt-2 w-64 backdrop-blur-2xl rounded-xl shadow-2xl overflow-hidden z-50 border border-white/20"
      role="menu"
      aria-orientation="vertical"
    >
      <!-- User Info -->
      <div class="p-4 border-b border-white/10">
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-xl bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-lg font-bold"
          >
            {$currentUser?.username?.charAt(0).toUpperCase() || "?"}
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-semibold truncate">
              {$currentUser?.username || "Utilisateur"}
            </p>
            <p class="text-xs text-gray-400">Connecté</p>
          </div>
        </div>
      </div>

      <!-- JWT Status -->
      <div class="p-2">
        <button
          on:click={handleUpdateJWT}
          class="w-full p-3 rounded-lg {statusConfig.color} border hover:opacity-80 transition-opacity text-left"
          role="menuitem"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <span class="text-lg font-bold">{statusConfig.icon}</span>
              <div>
                <p class="font-medium text-sm">{statusConfig.text}</p>
                {#if $currentUser?.jwtExpiresAt && jwtStatus === "valid"}
                  <p class="text-xs opacity-75">
                    Expire le {new Date(
                      $currentUser.jwtExpiresAt
                    ).toLocaleDateString()}
                  </p>
                {:else}
                  <p class="text-xs opacity-75">Cliquez pour mettre à jour</p>
                {/if}
              </div>
            </div>
            <svg
              class="w-4 h-4"
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
        </button>
      </div>

      <!-- Actions -->
      <div class="p-2 border-t border-white/10">
        <button
          on:click={handleLogout}
          class="w-full p-3 rounded-lg hover:bg-red-500/10 transition-colors text-left text-red-400 flex items-center gap-3"
          role="menuitem"
        >
          <svg
            class="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
            />
          </svg>
          <span>Déconnexion</span>
        </button>
      </div>
    </div>
  {/if}
</div>
