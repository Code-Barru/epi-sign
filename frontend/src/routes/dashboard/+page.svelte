<script lang="ts">
  import { onMount } from "svelte";
  import { loadUsers, signUsers, checkAuth } from "$lib/api";
  import { goto } from "$app/navigation";
  import { isMobileDevice } from "$lib/utils/device";
  import type { PublicUserResponse, ApiError } from "$lib/types";
  import AlertMessage from "$lib/components/AlertMessage.svelte";
  import SigningSection from "$lib/components/SigningSection.svelte";
  import UsersList from "$lib/components/UsersList.svelte";
  import QRScanner from "$lib/components/QRScanner.svelte";

  let users: PublicUserResponse[] = [];
  let selectedUsers = new Set<string>();
  let signUrl: string = "";
  let error: string = "";
  let success: string = "";
  let loading: boolean = false;
  let signing: boolean = false;
  let selectAll: boolean = false;
  let showScanner: boolean = false;
  let isMobile: boolean = false;

  onMount(async () => {
    // Vérifier si on est sur mobile
    isMobile = isMobileDevice();

    // Vérifier l'authentification d'abord
    const authenticated = await checkAuth();

    if (!authenticated) {
      goto("/login");
      return;
    }

    loading = true;
    try {
      users = await loadUsers();
    } catch (e) {
      const apiError = e as ApiError;
      if (apiError.status === 401) {
        goto("/login");
      } else {
        error = "Impossible de charger les utilisateurs";
      }
    } finally {
      loading = false;
    }
  });

  async function handleSign(): Promise<void> {
    error = "";
    success = "";

    if (selectedUsers.size === 0) {
      error = "Sélectionnez au moins un utilisateur";
      return;
    }

    if (!signUrl) {
      error = "Entrez une URL de signature";
      return;
    }

    signing = true;
    try {
      const result = await signUsers(Array.from(selectedUsers), signUrl);
      console.log("Résultat de la signature :", result);
      success = `Signature réussie : ${result}`;
      selectedUsers = new Set();
      selectAll = false;
      signUrl = "";
    } catch (e) {
      const apiError = e as ApiError;
      if (apiError.status === 404) {
        error = "Aucun cookie trouvé pour aujourd'hui";
      } else if (apiError.status === 400) {
        error = "Utilisateurs non trouvés";
      } else {
        error = "Erreur lors de la signature";
      }
    } finally {
      signing = false;
    }
  }

  function handleUserToggle(event: CustomEvent<string>): void {
    const userId = event.detail;
    if (selectedUsers.has(userId)) {
      selectedUsers.delete(userId);
    } else {
      selectedUsers.add(userId);
    }
    selectedUsers = selectedUsers;
    selectAll = selectedUsers.size === users.length;
  }

  function handleToggleSelectAll(): void {
    if (selectAll) {
      selectedUsers = new Set();
    } else {
      selectedUsers = new Set(users.map((u) => u.id));
    }
    selectAll = !selectAll;
  }

  function handleUrlChange(event: CustomEvent<string>): void {
    signUrl = event.detail;
  }

  function handleScanRequest(): void {
    showScanner = true;
  }

  function handleScanResult(result: string): void {
    // Vérifier si c'est une URL Epitech valide
    if (result.includes("intra.epitech.eu")) {
      signUrl = result;
      success = "QR Code scanné avec succès";
    } else {
      error = "QR Code invalide - URL Epitech attendue";
    }
  }

  function handleScanError(errorMsg: string): void {
    error = errorMsg;
  }
</script>

<div class="min-h-screen pb-safe">
  <div class="p-4 sm:p-6 max-w-7xl mx-auto">
    <h2 class="text-2xl sm:text-3xl font-bold mb-6 gradient-text">
      Signatures du jour
    </h2>

    {#if error}
      <AlertMessage type="error" message={error} />
    {/if}

    {#if success}
      <AlertMessage type="success" message={success} />
    {/if}

    <SigningSection
      {signUrl}
      {selectedUsers}
      {signing}
      {selectAll}
      {isMobile}
      usersCount={users.length}
      on:sign={handleSign}
      on:toggleSelectAll={handleToggleSelectAll}
      on:urlChange={handleUrlChange}
      on:scanRequest={handleScanRequest}
    />

    <UsersList
      {users}
      {selectedUsers}
      {loading}
      on:userToggle={handleUserToggle}
    />
  </div>
</div>

<!-- Scanner QR Code -->
{#if showScanner}
  <QRScanner
    onScan={handleScanResult}
    onError={handleScanError}
    onClose={() => (showScanner = false)}
  />
{/if}
