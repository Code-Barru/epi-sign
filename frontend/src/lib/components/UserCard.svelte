<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { PublicUserResponse } from "$lib/types";

  export let user: PublicUserResponse;
  export let isSelected: boolean;

  const dispatch = createEventDispatcher();

  user.jwtIsExpired =
    user.jwtExpiresAt === undefined ||
    user.jwtExpiresAt === null ||
    new Date(user.jwtExpiresAt) < new Date();

  function handleToggle() {
    if (!user.jwtIsExpired) {
      dispatch("toggle", user.id);
    }
  }
</script>

<label
  for="user-{user.id}"
  class="block glass-effect rounded-xl p-4 transition-all duration-200 {!user.jwtIsExpired
    ? 'cursor-pointer active:scale-[0.98]'
    : 'cursor-not-allowed opacity-50'}"
>
  <div class="flex items-center gap-3">
    <input
      type="checkbox"
      id="user-{user.id}"
      checked={isSelected}
      on:change={handleToggle}
      disabled={user.jwtIsExpired}
      class="checkbox-custom flex-shrink-0"
    />
    <div class="flex-1 min-w-0">
      <div class="font-semibold text-base">
        {user.username}
      </div>
      <div class="text-xs text-gray-400 truncate">
        {user.id}
      </div>
    </div>
    <div class="flex-shrink-0">
      <span
        class="inline-block px-2 py-1 {user.jwtIsExpired
          ? 'bg-red-500/20 text-red-400 border-red-500/30'
          : 'bg-green-500/20 text-green-400 border-green-500/30'} rounded-lg text-xs font-medium border"
      >
        {user.jwtIsExpired ? "JWT expiré" : "JWT valide"}
      </span>
    </div>
  </div>
</label>
