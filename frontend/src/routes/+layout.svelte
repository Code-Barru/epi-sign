<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import { isAuthenticated, currentUser } from "$lib/stores";
  import ProfileMenu from "$lib/components/ProfileMenu.svelte";
  import JWTUpdater from "$lib/components/JWTUpdater.svelte";

  let showJWTUpdater = false;
  let isMobile = false;

  // Détecter si on est sur mobile
  $: isMobile = typeof window !== "undefined" && window.innerWidth < 640;

  function handleUpdateJWT() {
    showJWTUpdater = true;
  }

  function handleCloseJWTUpdater() {
    showJWTUpdater = false;
  }
</script>

<div class="min-h-screen relative">
  <!-- Background gradient -->
  <div class="fixed inset-0 -z-10">
    <div
      class="absolute inset-0 bg-gradient-to-br from-purple-900/20 via-transparent to-pink-900/20"
    ></div>
    <div
      class="absolute top-20 -left-20 w-40 h-40 sm:w-96 sm:h-96 bg-purple-500/10 rounded-full blur-3xl"
    ></div>
    <div
      class="absolute bottom-20 -right-20 w-40 h-40 sm:w-96 sm:h-96 bg-pink-500/10 rounded-full blur-3xl"
    ></div>
  </div>

  <!-- Navbar -->
  {#if $isAuthenticated && $page.url.pathname !== "/login" && $page.url.pathname !== "/register"}
    <nav
      class="glass-effect border-b border-white/10 sticky top-0 z-50 safe-top"
    >
      <div class="px-4 py-3 sm:px-6 sm:py-4">
        <div class="flex justify-between items-center">
          <h1 class="text-xl sm:text-2xl font-bold gradient-text">EpiSign</h1>

          <!-- Profile Menu -->
          <ProfileMenu {isMobile} on:updateJWT={handleUpdateJWT} />
        </div>
      </div>
    </nav>
  {/if}

  <!-- Page content -->
  <main class="safe-bottom">
    <slot />
  </main>
</div>

<!-- JWT Updater Modal -->
<JWTUpdater isOpen={showJWTUpdater} on:close={handleCloseJWTUpdater} />
