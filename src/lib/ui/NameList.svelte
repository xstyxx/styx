<script lang="ts">
  import { flip } from "svelte/animate";

  import type { WsUser } from "$lib/protocol";
  import { nameToHue } from "./LiveCursor.svelte";

  export let users: [number, WsUser][];
  $: sortedUsers = [...users].sort(
    (a, b) => Number(b[1].canWrite) - Number(a[1].canWrite),
  );
</script>

<ul class="flex flex-col gap-1">
  {#each sortedUsers as [id, user] (id)}
    <li
      class={`flex p-1.5 gap-3 items-center rounded border border-zinc-800/50 bg-zinc-900/30 ${user.canWrite ? "" : "opacity-60"}`}
      animate:flip={{ duration: 250 }}
    >
      <div class="relative">
        <div
          style:background="hsl({nameToHue(user.name)}, 75%, 50%)"
          class="w-2.5 h-2.5 rounded-full"
        />
        <div
          style:background="hsl({nameToHue(user.name)}, 75%, 50%)"
          class="absolute inset-0 w-2.5 h-2.5 rounded-full animate-ping opacity-40"
        />
      </div>
      <div
        class="text-xs font-mono font-medium text-zinc-300 tracking-wide"
      >
        {user.name}
      </div>
      {#if !user.canWrite}
        <span class="text-[10px] font-mono text-zinc-500 ml-auto">RO</span>
      {/if}
    </li>
  {/each}
</ul>
