<script lang="ts">
  import { ArrowLeft, CheckCircle, Clock, Zap } from "@lucide/svelte";
  import { fly, fade } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { goto } from "$app/navigation";

  interface RoadmapItem {
    title: string;
    description: string;
    status: "planned" | "in-progress" | "completed";
    priority: "low" | "medium" | "high";
    category?: string;
  }

  const roadmapItems: RoadmapItem[] = [
    {
      title: "Scan de QR Code pour soit-même",
      description:
        "Ajouter la possibilité de scanner le QR Code et automatiquement signer pour soi-même sans action manuel",
      status: "completed",
      priority: "high",
      category: "Features",
    },
    {
      title: "Signer pour plusieurs personnes",
      description:
        "Permettre de signer pour plusieurs personnes en une seule fois en sélectionnant les utilisateurs",
      status: "completed",
      priority: "high",
      category: "Features",
    },
    {
      title: "Roadmap",
      description:
        "Créer une page de roadmap pour afficher les fonctionnalités à venir et leur statut",
      status: "completed",
      priority: "medium",
      category: "UI/UX",
    },
    {
      title: "Réserver un créneau pour signer",
      description:
        "Permettre de réserver un créneau de session pour être afficher en prioritaire dans la liste des utilisateurs",
      status: "planned",
      priority: "high",
      category: "Features",
    },
    {
      title: "Photo de profil",
      description:
        "Ajouter la possibilité de mettre une photo de profil pour chaque utilisateur",
      status: "planned",
      priority: "low",
      category: "Features",
    },
    {
      title: "Récupération automatique du JWT",
      description: "",
      status: "planned",
      priority: "medium",
      category: "Features",
    },
    {
      title: "Ajout de rôles et permissions",
      description:
        "Permettre de gérer les rôles et permissions des utilisateurs pour contrôler l'accès aux fonctionnalités",
      status: "planned",
      priority: "medium",
      category: "Security",
    },
  ];

  function goBack() {
    goto("/");
  }

  function getStatusConfig(status: RoadmapItem["status"]) {
    switch (status) {
      case "completed":
        return {
          icon: CheckCircle,
          color: "text-green-400 bg-green-500/10 border-green-500/30",
          label: "Terminé",
        };
      case "in-progress":
        return {
          icon: Zap,
          color: "text-blue-400 bg-blue-500/10 border-blue-500/30",
          label: "En cours",
        };
      case "planned":
        return {
          icon: Clock,
          color: "text-yellow-400 bg-yellow-500/10 border-yellow-500/30",
          label: "Planifié",
        };
    }
  }

  function getPriorityConfig(priority: RoadmapItem["priority"]) {
    switch (priority) {
      case "high":
        return {
          color: "text-red-400 bg-red-500/10 border-red-500/30",
          label: "Haute",
        };
      case "medium":
        return {
          color: "text-orange-400 bg-orange-500/10 border-orange-500/30",
          label: "Moyenne",
        };
      case "low":
        return {
          color: "text-gray-400 bg-gray-500/10 border-gray-500/30",
          label: "Basse",
        };
    }
  }

  // Grouper par statut pour l'affichage
  $: groupedItems = {
    completed: roadmapItems.filter((item) => item.status === "completed"),
    "in-progress": roadmapItems.filter((item) => item.status === "in-progress"),
    planned: roadmapItems.filter((item) => item.status === "planned"),
  };
</script>

<div class="min-h-screen pb-safe">
  <!-- En-tête avec navigation -->
  <div
    class="flex items-center justify-start p-4 sm:p-6"
    in:fly={{ y: -20, duration: 400, easing: quintOut }}
  >
    <button
      on:click={goBack}
      class="p-2 rounded-xl hover:bg-white/10 transition-all duration-200 ease-out transform hover:scale-110 active:scale-95"
      aria-label="Retour"
    >
      <ArrowLeft class="w-5 h-5 sm:w-6 sm:h-6" />
    </button>
    <div class="ml-3">
      <h1 class="text-xl sm:text-2xl font-bold gradient-text">Roadmap</h1>
      <p class="text-sm text-gray-400 mt-1">Fonctionnalités à venir</p>
    </div>
  </div>

  <div class="px-4 sm:px-6 max-w-4xl mx-auto">
    <!-- Roadmap par statut -->
    <div class="space-y-6 sm:space-y-8">
      <!-- En cours -->
      {#if groupedItems["in-progress"].length > 0}
        <section
          in:fly={{ y: 30, duration: 400, delay: 300, easing: quintOut }}
        >
          <div class="flex items-center gap-3 mb-4">
            <Zap class="w-5 h-5 text-blue-400" />
            <h3 class="text-lg font-semibold text-white">
              En cours de développement
            </h3>
          </div>
          <div class="space-y-3">
            {#each groupedItems["in-progress"] as item, index}
              {@const statusConfig = getStatusConfig(item.status)}
              {@const priorityConfig = getPriorityConfig(item.priority)}
              <div
                class="glass-effect-card rounded-xl p-4 sm:p-5 border-l-4 border-blue-400"
                in:fly={{
                  x: -20,
                  duration: 300,
                  delay: 350 + index * 50,
                  easing: quintOut,
                }}
              >
                <div
                  class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3"
                >
                  <div class="flex-1">
                    <div class="flex items-center gap-2 mb-2">
                      <h4 class="font-semibold text-white text-base sm:text-lg">
                        {item.title}
                      </h4>
                      {#if item.category}
                        <span
                          class="px-2 py-1 bg-white/10 text-gray-300 text-xs rounded-lg"
                        >
                          {item.category}
                        </span>
                      {/if}
                    </div>
                    <p
                      class="text-gray-300 text-sm sm:text-base leading-relaxed mb-3"
                    >
                      {item.description}
                    </p>
                    <div class="flex items-center gap-3 text-xs">
                      <span
                        class="px-2 py-1 rounded-lg border {statusConfig.color}"
                      >
                        {statusConfig.label}
                      </span>
                      <span
                        class="px-2 py-1 rounded-lg border {priorityConfig.color}"
                      >
                        Priorité {priorityConfig.label}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}

      <!-- Planifié -->
      {#if groupedItems.planned.length > 0}
        <section
          in:fly={{ y: 30, duration: 400, delay: 400, easing: quintOut }}
        >
          <div class="flex items-center gap-3 mb-4">
            <Clock class="w-5 h-5 text-yellow-400" />
            <h3 class="text-lg font-semibold text-white">Planifié</h3>
          </div>
          <div class="space-y-3">
            {#each groupedItems.planned as item, index}
              {@const statusConfig = getStatusConfig(item.status)}
              {@const priorityConfig = getPriorityConfig(item.priority)}
              <div
                class="glass-effect-card rounded-xl p-4 sm:p-5 border-l-4 border-yellow-400"
                in:fly={{
                  x: -20,
                  duration: 300,
                  delay: 450 + index * 50,
                  easing: quintOut,
                }}
              >
                <div
                  class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3"
                >
                  <div class="flex-1">
                    <div class="flex items-center gap-2 mb-2">
                      <h4 class="font-semibold text-white text-base sm:text-lg">
                        {item.title}
                      </h4>
                      {#if item.category}
                        <span
                          class="px-2 py-1 bg-white/10 text-gray-300 text-xs rounded-lg"
                        >
                          {item.category}
                        </span>
                      {/if}
                    </div>
                    <p
                      class="text-gray-300 text-sm sm:text-base leading-relaxed mb-3"
                    >
                      {item.description}
                    </p>
                    <div class="flex items-center gap-3 text-xs">
                      <span
                        class="px-2 py-1 rounded-lg border {statusConfig.color}"
                      >
                        {statusConfig.label}
                      </span>
                      <span
                        class="px-2 py-1 rounded-lg border {priorityConfig.color}"
                      >
                        Priorité {priorityConfig.label}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}

      <!-- Terminé -->
      {#if groupedItems.completed.length > 0}
        <section
          in:fly={{ y: 30, duration: 400, delay: 200, easing: quintOut }}
        >
          <div class="flex items-center gap-3 mb-4">
            <CheckCircle class="w-5 h-5 text-green-400" />
            <h3 class="text-lg font-semibold text-white">Terminé</h3>
          </div>
          <div class="space-y-3">
            {#each groupedItems.completed as item, index}
              {@const statusConfig = getStatusConfig(item.status)}
              {@const priorityConfig = getPriorityConfig(item.priority)}
              <div
                class="glass-effect-card rounded-xl p-4 sm:p-5 border-l-4 border-green-400"
                in:fly={{
                  x: -20,
                  duration: 300,
                  delay: 250 + index * 50,
                  easing: quintOut,
                }}
              >
                <div
                  class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3"
                >
                  <div class="flex-1">
                    <div class="flex items-center gap-2 mb-2">
                      <h4 class="font-semibold text-white text-base sm:text-lg">
                        {item.title}
                      </h4>
                      {#if item.category}
                        <span
                          class="px-2 py-1 bg-white/10 text-gray-300 text-xs rounded-lg"
                        >
                          {item.category}
                        </span>
                      {/if}
                    </div>
                    <p
                      class="text-gray-300 text-sm sm:text-base leading-relaxed mb-3"
                    >
                      {item.description}
                    </p>
                    <div class="flex items-center gap-3 text-xs">
                      <span
                        class="px-2 py-1 rounded-lg border {statusConfig.color}"
                      >
                        {statusConfig.label}
                      </span>
                      <span
                        class="px-2 py-1 rounded-lg border {priorityConfig.color}"
                      >
                        Priorité {priorityConfig.label}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}
    </div>

    <div
      class="mt-8 mb-4 sm:mt-12 text-center"
      in:fade={{ delay: 600, duration: 400 }}
    >
      <div class="glass-effect-card rounded-xl p-4 sm:p-6">
        <h4 class="font-semibold text-white mb-2">Une idée ? Un besoin ?</h4>
        <p class="text-gray-300 text-sm leading-relaxed">
          Ton retour est précieux ! N'hésite pas à contacter Antoine pour
          demander des nouvelles features ou signaler un bug.
        </p>
      </div>
    </div>
  </div>
</div>
