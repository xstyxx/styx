<script lang="ts">
  import {
    TerminalIcon,
    ShieldIcon,
    ZapIcon,
    LockIcon,
    GlobeIcon,
    ServerIcon,
    DownloadIcon,
    PackageIcon,
    GitBranchIcon,
  } from "svelte-feather-icons";

  import logotypeDark from "$lib/assets/logotype-dark.svg";
  import CopyableCode from "$lib/ui/CopyableCode.svelte";
  import DownloadLink from "$lib/ui/DownloadLink.svelte";

  declare const __SSHX_ORIGIN__: string;

  const origin =
    typeof __SSHX_ORIGIN__ === "string" && __SSHX_ORIGIN__
      ? __SSHX_ORIGIN__
      : typeof window !== "undefined"
        ? window.location.origin
        : "";

  const downloadBase = `${origin}/downloads/current`;
  const installCmd = `curl -sSf ${origin}/get | sh`;
  const installRunCmd = `curl -sSf ${origin}/get | sh -s run`;

  let deployEl: HTMLDivElement;

  function scrollToDeploy() {
    deployEl.scrollIntoView({ behavior: "smooth" });
  }
</script>

<div class="scanlines" />
<div class="tactical-grid fixed inset-0 z-0" />

<main
  class="max-w-screen-xl mx-auto px-4 md:px-8 lg:px-16 text-zinc-100 overflow-x-hidden relative z-10"
>
  <!-- Header -->
  <header class="mt-6 mb-4 sm:my-8 flex items-center gap-4">
    <img class="h-11" src={logotypeDark} alt="styx" />
    <div class="ml-auto flex items-center gap-3">
      <span class="status-badge">
        <span class="inline-block w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse" />
        <span>OPERATIONAL</span>
      </span>
      <span class="text-xs font-mono text-zinc-600">v0.4.1</span>
    </div>
  </header>

  <!-- Hero -->
  <div class="pt-12 md:pt-20 pb-8">
    <div class="hero-badge">
      <div class="radar-dot" />
      <span class="text-[10px] font-mono text-zinc-400 tracking-[0.2em] uppercase">Tactical Terminal Relay</span>
    </div>

    <h1 class="font-mono font-bold text-4xl sm:text-5xl md:text-6xl lg:text-7xl max-w-[18ch] py-3 mb-6 tracking-tight leading-[1.05]">
      Share shells.
      <br />
      <span class="title-gradient">Own the canvas.</span>
    </h1>

    <p class="text-base sm:text-lg text-zinc-500 max-w-[50ch] mb-5 font-mono leading-relaxed">
      Multiplayer terminal sessions on an infinite workspace.
      End-to-end encrypted. Self-hosted. Zero trace.
    </p>

    <div class="flex items-center gap-4 mb-10 font-mono text-xs text-zinc-600">
      <span class="flex items-center gap-1.5"><LockIcon size="12" /> E2E AES-128</span>
      <span class="text-zinc-800">///</span>
      <span class="flex items-center gap-1.5"><ZapIcon size="12" /> Sub-ms RTT</span>
      <span class="text-zinc-800">///</span>
      <span class="flex items-center gap-1.5"><ServerIcon size="12" /> Your Infra</span>
    </div>

    <!-- Terminal mockup -->
    <div class="terminal-preview">
      <div class="terminal-header">
        <div class="flex gap-1.5">
          <div class="w-2.5 h-2.5 rounded-full bg-red-500/70" />
          <div class="w-2.5 h-2.5 rounded-full bg-amber-500/30" />
          <div class="w-2.5 h-2.5 rounded-full bg-green-500/30" />
        </div>
        <span class="text-[10px] font-mono text-zinc-600 ml-3 tracking-wider">STYX SESSION</span>
      </div>
      <div class="terminal-body font-mono text-xs sm:text-sm leading-relaxed">
        <p><span class="text-red-500">$</span> <span class="text-zinc-500">curl -sSf</span> <span class="text-amber-400/80">{origin}/get</span> <span class="text-zinc-500">| sh</span></p>
        <p class="text-zinc-700 mt-1">info: downloading styx v0.4.1 (x86_64-linux-musl)...</p>
        <p class="text-green-500/80 mt-0.5">info: installed to /usr/local/bin/styx</p>
        <p class="mt-3"><span class="text-red-500">$</span> <span class="text-zinc-200">styx</span></p>
        <p class="mt-2 text-zinc-500">  <span class="text-green-500">styx</span> v0.4.1</p>
        <p class="mt-1 text-zinc-500">  <span class="text-green-400">➜</span>  Link:  <span class="text-cyan-400/80 underline">{origin}/s/k9Xm2#aE3kZ...</span></p>
        <p class="text-zinc-500">  <span class="text-green-400">➜</span>  Shell: <span class="text-zinc-600">/bin/bash</span></p>
        <p class="mt-2 text-zinc-700">  Waiting for connections... <span class="cursor-blink">_</span></p>
      </div>
    </div>

    <div class="flex flex-wrap gap-3 pt-10 pb-16 md:pb-24">
      <button class="cta-primary" on:click={scrollToDeploy}>
        DEPLOY
      </button>
      <a href="https://github.com/xstyxx/styx" target="_blank" rel="noreferrer" class="cta-secondary">
        SOURCE
      </a>
    </div>
  </div>

  <!-- Modules -->
  <div class="grid md:grid-cols-2 xl:grid-cols-3 gap-3 mb-32">
    <div class="module">
      <div class="module-header"><span class="module-id">01</span><TerminalIcon size="14" /></div>
      <h3>Infinite Canvas</h3>
      <p>Multiple terminals on a 2D workspace. Move, resize, arrange freely.</p>
    </div>
    <div class="module">
      <div class="module-header"><span class="module-id">02</span><LockIcon size="14" /></div>
      <h3>E2E Encrypted</h3>
      <p>Argon2id KDF + AES-128-CTR. The relay never sees your keystrokes.</p>
    </div>
    <div class="module">
      <div class="module-header"><span class="module-id">03</span><ZapIcon size="14" /></div>
      <h3>Rust Engine</h3>
      <p>Native server with gRPC + WebSocket transport. Sub-millisecond on local.</p>
    </div>
    <div class="module">
      <div class="module-header"><span class="module-id">04</span><GlobeIcon size="14" /></div>
      <h3>Live Collaboration</h3>
      <p>Real-time cursors, callsigns, and comms. Full situational awareness.</p>
    </div>
    <div class="module">
      <div class="module-header"><span class="module-id">05</span><ShieldIcon size="14" /></div>
      <h3>Self-Hosted</h3>
      <p>Your infrastructure. Your rules. Full operational control.</p>
    </div>
    <div class="module">
      <div class="module-header"><span class="module-id">06</span><ServerIcon size="14" /></div>
      <h3>Cross-Platform</h3>
      <p>Static binaries: Linux, macOS, Windows, FreeBSD. ARM64 + x86_64.</p>
    </div>
  </div>

  <!-- Deployment -->
  <h2
    bind:this={deployEl}
    class="mt-16 mb-10 font-mono font-bold text-2xl sm:text-3xl scroll-mt-16 text-zinc-100"
  >
    <span class="text-red-500">//</span> DEPLOYMENT
  </h2>

  <section class="deploy-section">
    <h3>
      <DownloadIcon size="14" class="text-red-500/60 inline-block mr-1.5 mb-0.5" />
      Linux / macOS
    </h3>
    <div class="deploy-content">
      <p class="mb-3 text-zinc-500">One-liner install:</p>
      <CopyableCode value={installCmd} />

      <p class="mt-8 mb-3 text-zinc-600">Direct download:</p>
      <div class="flex flex-wrap gap-2 mb-2">
        <DownloadLink href="{downloadBase}/styx-aarch64-apple-darwin.tar.gz">macOS ARM64</DownloadLink>
        <DownloadLink href="{downloadBase}/styx-x86_64-apple-darwin.tar.gz">macOS x86-64</DownloadLink>
      </div>
      <div class="flex flex-wrap gap-2 mb-2">
        <DownloadLink href="{downloadBase}/styx-aarch64-unknown-linux-musl.tar.gz">Linux ARM64</DownloadLink>
        <DownloadLink href="{downloadBase}/styx-x86_64-unknown-linux-musl.tar.gz">Linux x86-64</DownloadLink>
        <DownloadLink href="{downloadBase}/styx-armv7-unknown-linux-musleabihf.tar.gz">Linux ARMv7</DownloadLink>
      </div>
      <div class="flex flex-wrap gap-2">
        <DownloadLink href="{downloadBase}/styx-x86_64-unknown-freebsd.tar.gz">FreeBSD x86-64</DownloadLink>
      </div>
    </div>
  </section>

  <section class="deploy-section">
    <h3>
      <DownloadIcon size="14" class="text-red-500/60 inline-block mr-1.5 mb-0.5" />
      Windows
    </h3>
    <div class="deploy-content">
      <div class="flex flex-wrap gap-2">
        <DownloadLink href="{downloadBase}/styx-x86_64-pc-windows-gnu.zip">Windows x86-64</DownloadLink>
        <DownloadLink href="{downloadBase}/styx-i686-pc-windows-gnu.zip">Windows x86</DownloadLink>
      </div>
    </div>
  </section>

  <section class="deploy-section">
    <h3>
      <GitBranchIcon size="14" class="text-red-500/60 inline-block mr-1.5 mb-0.5" />
      CI / Pipeline
    </h3>
    <div class="deploy-content">
      <p class="mb-3 text-zinc-500">Drop into a running pipeline:</p>
      <CopyableCode value={installRunCmd} />
    </div>
  </section>

  <!-- Footer -->
  <hr />
  <footer class="py-8 flex items-center justify-center gap-4">
    <span class="inline-block w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse" />
    <span class="font-mono text-xs text-zinc-600 tracking-wider">STYX RELAY — OPERATIONAL</span>
  </footer>
</main>

<style lang="postcss">
  :global(body) {
    background: #030303 !important;
  }

  .scanlines {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 2;
    background: repeating-linear-gradient(
      0deg,
      transparent,
      transparent 2px,
      rgba(255, 0, 0, 0.012) 2px,
      rgba(255, 0, 0, 0.012) 4px
    );
  }

  .status-badge {
    @apply flex items-center gap-1.5 px-2 py-0.5 rounded border border-green-500/20 bg-green-500/5 text-[10px] font-mono text-green-500/70 tracking-wider;
  }

  .hero-badge {
    @apply flex items-center gap-2 mb-8 px-3 py-1.5 rounded border border-red-500/20 bg-red-500/5 w-fit;
  }

  .radar-dot {
    @apply w-2 h-2 bg-red-500 rounded-full;
    animation: pulse-glow 2s ease-in-out infinite;
  }

  .title-gradient {
    background: linear-gradient(135deg, #ef4444 0%, #f97316 40%, #ef4444 100%);
    background-size: 200% 200%;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    animation: gradient-shift 5s ease infinite;
  }

  @keyframes gradient-shift {
    0%, 100% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
  }

  .terminal-preview {
    @apply rounded-md border border-zinc-800/60 overflow-hidden max-w-2xl;
    background: #080808;
    box-shadow: 0 0 80px rgba(239, 68, 68, 0.04), 0 4px 40px rgba(0, 0, 0, 0.5);
  }

  .terminal-header {
    @apply flex items-center px-4 py-2.5 border-b border-zinc-800/40;
    background: #060606;
  }

  .terminal-body {
    @apply p-4;
  }

  .cursor-blink {
    animation: blink 1s step-end infinite;
  }

  @keyframes blink {
    0%, 100% { opacity: 1; }
    50% { opacity: 0; }
  }

  .cta-primary {
    @apply font-mono font-bold text-sm px-7 py-2.5 rounded-md transition-all duration-200 tracking-wider;
    background: linear-gradient(135deg, #dc2626, #991b1b);
    color: white;
    border: 1px solid rgba(239, 68, 68, 0.25);
    box-shadow: 0 0 20px rgba(239, 68, 68, 0.12);
  }

  .cta-primary:hover {
    box-shadow: 0 0 30px rgba(239, 68, 68, 0.25);
    border-color: rgba(239, 68, 68, 0.5);
    transform: translateY(-1px);
  }

  .cta-primary:active {
    transform: translateY(0);
  }

  .cta-secondary {
    @apply font-mono font-medium text-sm px-7 py-2.5 rounded-md transition-all duration-200 tracking-wider;
    @apply border border-zinc-800 text-zinc-500 hover:border-zinc-600 hover:text-zinc-300;
  }

  hr {
    @apply border-zinc-900 mt-24;
  }

  /* Modules (feature blocks) */
  .module {
    @apply relative rounded-md p-5 transition-all duration-300;
    background: rgba(8, 8, 8, 0.8);
    border: 1px solid rgba(63, 63, 70, 0.2);
  }

  .module:hover {
    border-color: rgba(239, 68, 68, 0.15);
    box-shadow: 0 0 20px rgba(239, 68, 68, 0.04);
  }

  .module-header {
    @apply flex items-center gap-2 mb-3 text-red-500/60;
  }

  .module-id {
    @apply font-mono text-[10px] text-zinc-600 tracking-wider;
  }

  .module h3 {
    @apply font-mono font-bold text-sm text-zinc-200 mb-1.5;
  }

  .module p {
    @apply text-zinc-500 text-xs leading-relaxed;
  }

  /* Deployment sections */
  .deploy-section {
    @apply grid sm:grid-cols-[160px,1fr] gap-x-8 gap-y-3 max-w-4xl mx-auto py-6;
    @apply mb-8 border-t border-zinc-900/80;
  }

  .deploy-section h3 {
    @apply text-sm font-mono text-zinc-300;
  }

  .deploy-content {
    @apply text-sm font-mono;
  }
</style>
