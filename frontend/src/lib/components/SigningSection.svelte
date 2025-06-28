<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { ScanQrCode } from "@lucide/svelte";

  export let signUrl: string;
  export let selectedUsers: Set<string>;
  export let signing: boolean;
  export let selectAll: boolean;
  export let usersCount: number;
  export let isMobile: boolean = false;

  const dispatch = createEventDispatcher();

  function handleSign() {
    dispatch("sign");
  }

  function handleUrlChange(event: Event) {
    const target = event.target as HTMLInputElement;
    dispatch("urlChange", target.value);
  }

  function handleScanRequest() {
    dispatch("scanRequest");
  }
</script>

<div
  class="top-16 sm:top-20 z-40 -qmx-4 px-4 sm:mx-0 sm:px-0 py-4 glass-effect sm:glass-effect-none backdrop-blur-2xl sm:backdrop-blur-none mb-6"
>
  <div class="space-y-4">
    <div>
      <label for="signUrl" class="block text-sm font-medium text-gray-300 mb-2">
        URL de signature Epitech
      </label>
      <div class="flex gap-2">
        <input
          type="text"
          id="signUrl"
          value={signUrl}
          on:input={handleUrlChange}
          placeholder="https://intra.epitech.eu/..."
          class="input-field flex-1"
        />
        {#if isMobile}
          <button
            on:click={handleScanRequest}
            class="cursor-pointer px-4 bg-white/10 rounded-xl justify-center align-center"
            title="Scanner QR Code"
            aria-label="Scanner QR Code"
          >
            <ScanQrCode />
          </button>
        {/if}
      </div>
    </div>
    <div class="flex gap-3">
      <button
        on:click={handleSign}
        disabled={signing || selectedUsers.size === 0 || !signUrl}
        class="btn-primary flex-1"
      >
        {#if signing}
          <span
            class="inline-block animate-spin rounded-full h-5 w-5 border-b-2 border-white"
          ></span>
        {:else}
          Signer ({selectedUsers.size})
        {/if}
      </button>
    </div>
  </div>
</div>
