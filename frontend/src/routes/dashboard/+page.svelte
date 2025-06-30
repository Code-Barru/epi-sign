<script lang="ts">
  import { onMount } from "svelte";
  import { loadUsers, signUsers, checkAuth } from "$lib/api";
  import { goto } from "$app/navigation";
  import { isMobileDevice } from "$lib/utils/device";
  import type {
    PublicUserResponse,
    ApiError,
    UserSignResponse,
  } from "$lib/types";
  import AlertMessage from "$lib/components/AlertMessage.svelte";
  import SigningSection from "$lib/components/SigningSection.svelte";
  import UsersList from "$lib/components/UsersList.svelte";
  import QRScanner from "$lib/components/QRScanner.svelte";
  import SignResults from "$lib/components/SignResults.svelte";
  import { currentUser } from "$lib/stores";

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
  let showResults: boolean = false;
  let signResults: UserSignResponse[] = [];

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
      const loadedUsers = await loadUsers();

      // Marquer les utilisateurs avec JWT expiré
      loadedUsers.forEach((user) => {
        user.jwtIsExpired =
          user.jwtExpiresAt === undefined ||
          user.jwtExpiresAt === null ||
          new Date(user.jwtExpiresAt) < new Date();
      });

      // Trier les utilisateurs :
      // 1. Utilisateur actuel en premier
      // 2. Par validité de token (valides avant expirés)
      // 3. Par ULID
      users = loadedUsers.sort((a, b) => {
        // 1. Utilisateur actuel en premier
        const aIsCurrent = $currentUser?.id === a.id;
        const bIsCurrent = $currentUser?.id === b.id;

        if (aIsCurrent && !bIsCurrent) return -1;
        if (!aIsCurrent && bIsCurrent) return 1;

        // 2. Trier par validité de token (valides en premier)
        if (a.jwtIsExpired !== b.jwtIsExpired) {
          return a.jwtIsExpired ? 1 : -1;
        }

        // 3. Trier par ULID
        return a.id.localeCompare(b.id);
      });
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
    showResults = false;
    signResults = [];

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
      const selectedUserIds = Array.from(selectedUsers);

      // L'API retourne maintenant directement un array de UserSignResponse
      const response = await signUsers(selectedUserIds, signUrl);

      // Response est déjà au bon format : UserSignResponse[]
      signResults = response;

      showResults = true;

      // Réinitialiser la sélection
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

    // Mettre à jour selectAll en fonction de la sélection
    const availableUsers = users.filter((u) => !u.jwtIsExpired);
    selectAll =
      selectedUsers.size === availableUsers.length && availableUsers.length > 0;
  }

  function handleToggleSelectAll(): void {
    const availableUsers = users.filter((u) => !u.jwtIsExpired);

    if (selectAll) {
      // Désélectionner tous
      selectedUsers = new Set();
      selectAll = false;
    } else {
      // Sélectionner tous les utilisateurs disponibles
      selectedUsers = new Set(availableUsers.map((u) => u.id));
      selectAll = true;
    }
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

  function handleCloseResults() {
    showResults = false;
  }
</script>

<div class="min-h-screen pb-safe">
  <div class="p-4 sm:p-6 max-w-7xl mx-auto">
    {#if error}
      <AlertMessage type="error" message={error} />
    {/if}

    {#if success}
      <AlertMessage type="success" message={success} />
    {/if}

    <!-- Layout Mobile : vertical (comme avant) -->
    <!-- Layout Desktop : horizontal avec SigningSection à gauche (1/4) et UsersList à droite (3/4) -->
    <div class="flex flex-col sm:flex-row sm:gap-6 sm:h-[calc(100vh-120px)]">
      <!-- Section de signature - Mobile: pleine largeur, Desktop: 1/4 à gauche -->
      <div class="sm:w-1/4 sm:flex-shrink-0">
        <SigningSection
          {signUrl}
          {selectedUsers}
          {signing}
          {isMobile}
          on:sign={handleSign}
          on:toggleSelectAll={handleToggleSelectAll}
          on:urlChange={handleUrlChange}
          on:scanRequest={handleScanRequest}
        />
      </div>

      <!-- Liste des utilisateurs - Mobile: pleine largeur, Desktop: 3/4 à droite -->
      <div class="flex-1 sm:overflow-hidden sm:pl-2">
        <div class="sm:h-full sm:overflow-y-auto sm:overflow-x-hidden">
          <UsersList
            {users}
            {selectedUsers}
            {loading}
            on:userToggle={handleUserToggle}
          />
        </div>
      </div>
    </div>
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

<SignResults
  isOpen={showResults}
  results={signResults}
  {users}
  on:close={handleCloseResults}
/>
