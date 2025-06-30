<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { ScanQrCode } from "@lucide/svelte";

  export let signUrl: string;
  export let selectedUsers: Set<string>;
  export let signing: boolean;
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
  class="top-16 sm:top-20 z-40 -mx-1 px-4 py-4 rounded-xl bg-gray-900/95 border-b border-gray-700/50 sm:glass-effect-navbar sm:backdrop-blur-2xl sm:border-white/10 mb-6"
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
            class="cursor-pointer px-4 bg-gray-700/80 hover:bg-gray-700/90 sm:bg-white/10 sm:hover:bg-white/20 rounded-xl justify-center align-center transition-all duration-200 ease-out transform hover:scale-105 active:scale-95"
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
