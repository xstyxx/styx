<script lang="ts">
  import { fade } from "svelte/transition";

  export let status: "connected" | "no-server" | "no-shell";
  export let serverLatency: number | null;
  export let shellLatency: number | null;

  function displayLatency(latency: number) {
    if (latency < 1) return "<1ms";
    else if (latency <= 950) return `${Math.round(latency)}ms`;
    else return `${(latency / 1000).toFixed(1)}s`;
  }

  function colorLatency(latency: number | null) {
    if (latency === null) return "text-zinc-500";
    else if (latency < 80) return "text-green-400";
    else if (latency < 300) return "text-amber-400";
    else return "text-red-400";
  }
</script>

<div
  class="relative panel p-4"
  in:fade|local={{ duration: 100 }}
  out:fade|local={{ duration: 75 }}
>
  <div class="absolute left-[calc(50%-8px)] top-[-16px] w-4 h-4">
    <svg viewBox="0 0 16 16">
      <path d="M 0 12 L 8 0 L 16 12 Z" fill="#0a0a0a" stroke="#27272a" />
    </svg>
  </div>

  <h2 class="font-mono text-xs tracking-widest text-zinc-500 uppercase mb-3 text-center">SIGNAL</h2>

  <p class="text-zinc-300 text-sm text-center font-mono">
    {#if status === "connected"}
      {#if serverLatency === null || shellLatency === null}
        <span class="text-amber-400">CALIBRATING...</span>
      {:else}
        RTT: <span class="text-green-400">{displayLatency(serverLatency + shellLatency)}</span>
      {/if}
    {:else}
      <span class="text-red-400">DISCONNECTED</span>
    {/if}
  </p>

  <div class="flex justify-between items-center mt-5">
    <div class="node active" />
    <div class="link" />
    <div class="node" class:active={status !== "no-server"} />
    <div class="link" />
    <div class="node" class:active={status === "connected"} />
  </div>

  <div class="flex justify-between items-center mt-2">
    <p class="text-xs font-mono text-zinc-400 w-10">YOU</p>

    {#if status === "connected"}
      <p class="text-xs font-mono w-12 text-left {colorLatency(serverLatency)}">
        {#if serverLatency !== null}{displayLatency(serverLatency)}{/if}
      </p>
    {/if}

    <p class="text-xs font-mono text-zinc-400 text-center">RELAY</p>

    {#if status === "connected"}
      <p class="text-xs font-mono w-12 text-right {colorLatency(shellLatency)}">
        {#if shellLatency !== null}{displayLatency(shellLatency)}{/if}
      </p>
    {/if}

    <p class="text-xs font-mono text-zinc-400 w-10 text-right">SHELL</p>
  </div>
</div>

<style lang="postcss">
  .node {
    @apply rounded-full w-3 h-3 border-2 border-zinc-700 transition-all duration-300;
  }

  .node.active {
    @apply border-green-500/80 bg-green-500/30;
    box-shadow: 0 0 6px rgba(34, 197, 94, 0.4);
  }

  .link {
    @apply flex-1 mx-1 border-t border-dashed border-zinc-700;
  }
</style>
