<script lang="ts" context="module">
  import type { WsUser } from "$lib/protocol";

  export function nameToHue(name: string): number {
    let hash = 2166136261;
    for (let i = 0; i < name.length; i++) {
      hash = (hash ^ name.charCodeAt(i)) * 16777619;
    }
    hash = (hash * 16777619) ^ -1;
    return 360 * (hash / (1 << 31));
  }
</script>

<script lang="ts">
  import { fade } from "svelte/transition";

  export let user: WsUser;
  export let showName = false;

  let hovering = false;
  let lastMove = Date.now();

  let lastCursor: [number, number] | null = null;
  let time = Date.now();
  $: if (
    !lastCursor ||
    (user.cursor &&
      (lastCursor[0] !== user.cursor[0] || lastCursor[1] != user.cursor[1]))
  ) {
    lastCursor = user.cursor;
    lastMove = Date.now();
    setTimeout(() => {
      time = Date.now();
    }, 4000);
  }
</script>

<div
  class="flex items-start"
  on:mouseenter={() => (hovering = true)}
  on:mouseleave={() => (hovering = false)}
>
  <svg width="20" height="20" viewBox="0 0 23 23" style="filter: drop-shadow(0 0 3px hsl({nameToHue(user.name)}, 100%, 50%));">
    <path
      d="M11 22L2 2L22 11L14 14Z"
      fill="hsl({nameToHue(user.name)}, 100%, 50%)"
      stroke="white"
      stroke-width="0.8"
    />
  </svg>
  {#if showName || hovering || time - lastMove < 3500}
    <p
      class="mt-3 ml-0.5 font-mono text-[10px] px-1.5 py-0.5 rounded-sm border text-white tracking-wide shadow-lg whitespace-nowrap"
      style="background: hsl({nameToHue(user.name)}, 50%, 20%); border-color: hsl({nameToHue(user.name)}, 70%, 35%);"
      transition:fade|local={{ duration: 150 }}
    >
      {user.name}
    </p>
  {/if}
</div>
