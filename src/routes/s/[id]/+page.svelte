<script lang="ts">
  import { page } from "$app/stores";
  import Session from "$lib/Session.svelte";

  let title: string = "styx";
</script>

<svelte:head>
  <title>{title}</title>
  <style>
    body {
      overscroll-behavior: none;
      background: #0c0c10;
    }
  </style>
</svelte:head>

<div class="session-bg"></div>
<div class="session-scanlines"></div>
<div class="session-vignette"></div>

<Session
  id={$page.params.id}
  on:receiveName={({ detail: sessionName }) => {
    if (sessionName) {
      title = `${sessionName} — styx`;
    }
  }}
/>

<style>
  .session-bg {
    position: fixed;
    inset: 0;
    z-index: -20;
    background:
      linear-gradient(180deg, #0c0c10 0%, #100a0e 50%, #0c0c10 100%),
      repeating-linear-gradient(
        0deg,
        transparent,
        transparent 39px,
        rgba(239, 68, 68, 0.04) 39px,
        rgba(239, 68, 68, 0.04) 40px
      ),
      repeating-linear-gradient(
        90deg,
        transparent,
        transparent 39px,
        rgba(239, 68, 68, 0.04) 39px,
        rgba(239, 68, 68, 0.04) 40px
      );
  }

  .session-scanlines {
    position: fixed;
    inset: 0;
    z-index: -10;
    background: repeating-linear-gradient(
      0deg,
      transparent,
      transparent 2px,
      rgba(0, 0, 0, 0.08) 2px,
      rgba(0, 0, 0, 0.08) 4px
    );
    pointer-events: none;
  }

  .session-vignette {
    position: fixed;
    inset: 0;
    z-index: -5;
    background: radial-gradient(
      ellipse at center,
      transparent 40%,
      rgba(0, 0, 0, 0.4) 100%
    );
    pointer-events: none;
  }
</style>
