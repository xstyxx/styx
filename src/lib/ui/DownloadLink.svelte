<script lang="ts">
  import { ExternalLinkIcon } from "svelte-feather-icons";

  export let href: string;
</script>

<a
  {href}
  class="flex items-baseline gap-1.5 py-1 px-2 rounded-lg bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 hover:border-red-500/20 text-zinc-400 hover:text-zinc-200 transition-all font-mono"
>
  <span class="text-sm">
    <slot />
  </span>
  <ExternalLinkIcon size="12" class="text-zinc-600" />
</a>
