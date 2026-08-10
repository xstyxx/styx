<script lang="ts">
  import { ChevronDownIcon } from "svelte-feather-icons";

  import { settings, updateSettings } from "$lib/settings";
  import OverlayMenu from "./OverlayMenu.svelte";
  import themes, { type ThemeName } from "./themes";

  export let open: boolean;

  let inputName: string;
  let inputTheme: ThemeName;
  let inputScrollback: number;

  let initialized = false;
  $: open, (initialized = false);
  $: if (!initialized) {
    initialized = true;
    inputName = $settings.name;
    inputTheme = $settings.theme;
    inputScrollback = $settings.scrollback;
  }
</script>

<OverlayMenu
  title="CONFIGURATION"
  description="Terminal and display settings."
  showCloseButton
  {open}
  on:close
>
  <div class="flex flex-col gap-3">
    <div class="item">
      <div>
        <p class="item-title">Callsign</p>
        <p class="item-subtitle">How you appear to other operators.</p>
      </div>
      <div>
        <input
          class="input-common"
          placeholder="CALLSIGN"
          bind:value={inputName}
          maxlength="50"
          on:input={() => {
            if (inputName.length >= 2) {
              updateSettings({ name: inputName });
            }
          }}
        />
      </div>
    </div>
    <div class="item">
      <div>
        <p class="item-title">Color Palette</p>
        <p class="item-subtitle">Terminal color scheme.</p>
      </div>
      <div class="relative">
        <ChevronDownIcon
          class="absolute top-[11px] right-2.5 w-4 h-4 text-zinc-500"
        />
        <select
          class="input-common !pr-5"
          bind:value={inputTheme}
          on:change={() => updateSettings({ theme: inputTheme })}
        >
          {#each Object.keys(themes) as themeName (themeName)}
            <option value={themeName}>{themeName}</option>
          {/each}
        </select>
      </div>
    </div>
    <div class="item">
      <div>
        <p class="item-title">Scrollback</p>
        <p class="item-subtitle">Buffer depth (lines).</p>
      </div>
      <div>
        <input
          type="number"
          class="input-common"
          bind:value={inputScrollback}
          on:input={() => {
            if (inputScrollback >= 0) {
              updateSettings({ scrollback: inputScrollback });
            }
          }}
          step="100"
        />
      </div>
    </div>
  </div>

  <!-- svelte-ignore missing-declaration -->
  <p class="mt-6 text-xs text-right font-mono text-zinc-600">
    styx-server v{__APP_VERSION__}
  </p>
</OverlayMenu>

<style lang="postcss">
  .item {
    @apply rounded-md p-4 flex gap-4 flex-col sm:flex-row items-start;
    background: rgba(10, 10, 10, 0.6);
    border: 1px solid rgba(63, 63, 70, 0.3);
  }

  .item > div:first-child {
    @apply flex-1;
  }

  .item-title {
    @apply font-mono font-medium text-zinc-200 text-sm mb-0.5;
  }

  .item-subtitle {
    @apply text-xs text-zinc-500;
  }

  .input-common {
    @apply w-48 px-3 py-2 text-sm font-mono rounded-md bg-zinc-950;
    @apply border border-zinc-800 outline-none focus:ring-1 focus:ring-red-500/40 focus:border-red-500/30;
    @apply appearance-none transition-all;
  }
</style>
