<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import {
    MessageSquareIcon,
    PlusCircleIcon,
    SettingsIcon,
    WifiIcon,
  } from "svelte-feather-icons";

  import logo from "$lib/assets/logo.svg";

  export let connected: boolean;
  export let hasWriteAccess: boolean | undefined;
  export let newMessages: boolean;

  const dispatch = createEventDispatcher<{
    create: void;
    chat: void;
    settings: void;
    networkInfo: void;
  }>();
</script>

<div class="panel inline-block px-3 py-2">
  <div class="flex items-center select-none">
    <a href="/" class="flex-shrink-0"
      ><img src={logo} alt="styx" class="h-9" /></a
    >
    <div class="ml-1.5 mr-2 flex items-center gap-2">
      <p class="font-mono font-bold text-sm text-zinc-300 tracking-wide">styx</p>
      {#if connected}
        <span class="live-dot" />
      {/if}
    </div>

    <div class="v-divider" />

    <div class="flex space-x-0.5">
      <button
        class="icon-button"
        on:click={() => dispatch("create")}
        disabled={!connected || !hasWriteAccess}
        title={!connected
          ? "Not connected"
          : hasWriteAccess === false
          ? "No write access"
          : "New terminal"}
      >
        <PlusCircleIcon strokeWidth={1.5} class="p-0.5" />
      </button>
      <button class="icon-button" on:click={() => dispatch("chat")}>
        <MessageSquareIcon strokeWidth={1.5} class="p-0.5" />
        {#if newMessages}
          <div class="activity" />
        {/if}
      </button>
      <button class="icon-button" on:click={() => dispatch("settings")}>
        <SettingsIcon strokeWidth={1.5} class="p-0.5" />
      </button>
    </div>

    <div class="v-divider" />

    <div class="flex space-x-0.5">
      <button class="icon-button" on:click={() => dispatch("networkInfo")}>
        <WifiIcon strokeWidth={1.5} class="p-0.5" />
      </button>
    </div>
  </div>
</div>

<style lang="postcss">
  .v-divider {
    @apply h-4 mx-2;
    border-left: 1px dashed rgba(63, 63, 70, 0.6);
  }

  .icon-button {
    @apply relative rounded-md p-1 transition-all duration-150;
    @apply disabled:opacity-40 disabled:bg-transparent;
  }

  .icon-button:hover:not(:disabled) {
    background: rgba(239, 68, 68, 0.08);
    box-shadow: 0 0 8px rgba(239, 68, 68, 0.1);
  }

  .icon-button:active:not(:disabled) {
    background: rgba(239, 68, 68, 0.15);
  }

  .activity {
    @apply absolute top-0.5 right-0.5 p-[3.5px] bg-red-500 rounded-full;
    animation: pulse-glow 2s ease-in-out infinite;
  }

  .live-dot {
    @apply inline-block w-1.5 h-1.5 rounded-full bg-green-500;
    box-shadow: 0 0 4px rgba(34, 197, 94, 0.6);
    animation: pulse-glow-green 2s ease-in-out infinite;
  }

  @keyframes pulse-glow-green {
    0%, 100% { box-shadow: 0 0 4px rgba(34, 197, 94, 0.4); }
    50% { box-shadow: 0 0 10px rgba(34, 197, 94, 0.8); }
  }
</style>
