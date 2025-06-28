<script lang="ts">
  export let type: "error" | "success";
  export let message: string;
</script>

<div
  class="mb-4 p-4 {type === 'error'
    ? 'bg-red-500/10 border border-red-500/50 text-red-400'
    : 'bg-green-500/10 border border-green-500/50 text-green-400'} 
    rounded-xl text-sm"
>
  {#if type === "error"}❌{/if}
  {message}
</div>
