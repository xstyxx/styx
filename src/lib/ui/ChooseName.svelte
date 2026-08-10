<script lang="ts">
  import { browser } from "$app/environment";
  import OverlayMenu from "./OverlayMenu.svelte";
  import { settings, updateSettings } from "$lib/settings";

  let value = "";

  function handleSubmit() {
    updateSettings({ name: value });
  }
</script>

<OverlayMenu
  title="Join Session"
  description="Enter your callsign before joining this session."
  maxWidth={560}
  open={browser && !$settings.name}
>
  <form class="flex gap-2" on:submit|preventDefault={handleSubmit}>
    <input
      class="flex-1 w-full px-3 py-2 rounded-md outline-none text-zinc-200 bg-zinc-900 border border-zinc-700 font-mono text-sm focus:ring-2 focus:ring-red-500/50 focus:border-red-500/30 placeholder:text-zinc-600"
      placeholder="Your name"
      required
      minlength="2"
      maxlength="50"
      bind:value
    />
    <button
      class="flex-shrink-0 px-4 py-2 bg-red-700 hover:bg-red-600 active:ring-2 active:ring-red-500/50 rounded-md font-mono font-bold text-sm tracking-wider transition-colors"
      >Join</button
    >
  </form>
</OverlayMenu>
