<script lang="ts">
  import { CheckIcon, CopyIcon } from "svelte-feather-icons";

  export let value: string;

  let copied = false;

  async function handleClick() {
    await navigator.clipboard.writeText(value);
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 1000);
  }
</script>

<div class="flex items-center gap-3 px-4 py-3 rounded-lg bg-zinc-900/80 border border-zinc-800">
  <code class="text-zinc-200 font-mono text-sm flex-1">{value}</code>
  <button
    class={"rounded p-1.5 transition-colors " +
      (!copied ? "hover:bg-red-500/10 text-zinc-500" : "text-green-400")}
    on:click={handleClick}
  >
    {#if copied}
      <CheckIcon size="16" />
    {:else}
      <CopyIcon size="16" />
    {/if}
  </button>
</div>
