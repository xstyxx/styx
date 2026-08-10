<script lang="ts">
  import { page } from "$app/stores";
  import logotypeDark from "$lib/assets/logotype-dark.svg";
</script>

<div class="tactical-grid fixed inset-0 -z-10" />

<main class="p-4 max-w-xl mx-auto my-6 md:my-12 lg:my-24">
  <img class="h-12 -mx-1" src={logotypeDark} alt="styx" />

  <div class="mt-8 mb-8 font-mono">
    {#if $page.status === 404}
      <div class="flex items-center gap-3 mb-4">
        <span class="inline-block px-2 py-0.5 bg-red-500/10 border border-red-500/30 rounded text-red-400 text-xs font-bold">404</span>
        <span class="text-zinc-300 text-sm">Session not found or expired.</span>
      </div>
      <p class="text-zinc-500 text-sm">The target session does not exist on this relay.</p>
    {:else}
      <div class="flex items-center gap-3 mb-4">
        <span class="inline-block px-2 py-0.5 bg-red-500/10 border border-red-500/30 rounded text-red-400 text-xs font-bold">{$page.status}</span>
        <span class="text-zinc-300 text-sm">Operation failed.</span>
      </div>
      <pre class="whitespace-pre-wrap break-all p-3 rounded-md bg-zinc-950 border border-zinc-800 text-xs text-zinc-400 font-mono">{JSON.stringify($page.error, null, 2)}</pre>
    {/if}
  </div>

  <a
    href="/"
    class="inline-block font-mono font-bold text-sm px-5 py-2 rounded-md bg-red-900/30 hover:bg-red-900/50 border border-red-500/20 hover:border-red-500/40 text-zinc-200 transition-all"
    >RETURN TO BASE</a
  >
</main>
