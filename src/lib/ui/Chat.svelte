<script lang="ts" context="module">
  export type ChatMessage = {
    uid: number;
    name: string;
    msg: string;
    sentAt: Date;
  };
</script>

<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { SendIcon } from "svelte-feather-icons";

  import CircleButton from "./CircleButton.svelte";
  import CircleButtons from "./CircleButtons.svelte";

  const dispatch = createEventDispatcher<{ chat: string; close: void }>();

  export let userId: number;
  export let messages: ChatMessage[];

  let groupedMessages: ChatMessage[][];
  $: {
    groupedMessages = [];
    let lastSender = -1;
    for (const chat of messages) {
      if (chat.uid === lastSender) {
        groupedMessages[groupedMessages.length - 1].push(chat);
      } else {
        groupedMessages.push([chat]);
        lastSender = chat.uid;
      }
    }
  }

  let scroller: HTMLElement;
  $: if (scroller && groupedMessages.length) {
    tick().then(() => {
      scroller.scroll({ top: scroller.scrollHeight });
    });
  }

  let text: string;

  function handleSubmit() {
    if (text) {
      dispatch("chat", text);
      text = "";
    }
  }
</script>

<div
  class="panel flex flex-col h-full max-h-[480px]"
  in:fade|local={{ duration: 100 }}
  out:fade|local={{ duration: 75 }}
>
  <div class="flex items-center p-3 border-b border-zinc-800/60">
    <CircleButtons>
      <CircleButton kind="red" on:click={() => dispatch("close")} />
    </CircleButtons>
    <div class="ml-3 text-zinc-400 text-xs font-mono tracking-wider uppercase">COMMS</div>
  </div>

  <div class="px-3 py-2 flex-1 overflow-y-auto" bind:this={scroller}>
    <div class="space-y-3">
      {#each groupedMessages as chatGroup}
        <div class="message-group" class:from-me={userId === chatGroup[0].uid}>
          <aside class="pl-2.5 text-zinc-500 text-xs font-mono">
            {chatGroup[0].name}
          </aside>
          {#each chatGroup as chat (chat)}
            <div
              class="chat"
              title="{chat.sentAt.toLocaleTimeString()}"
            >
              {chat.msg}
            </div>
          {/each}
        </div>
      {/each}
    </div>
  </div>

  <form class="relative p-3 border-t border-zinc-800/40" on:submit|preventDefault={handleSubmit}>
    <input
      class="w-full rounded-md bg-zinc-900 border border-zinc-800 pl-3.5 pr-9 py-1.5 outline-none text-zinc-300 font-mono text-sm focus:ring-1 focus:ring-red-500/40 focus:border-red-500/30 placeholder:text-zinc-600"
      placeholder="..."
      bind:value={text}
    />
    {#if text}
      <button
        class="absolute w-4 h-4 top-[22px] right-[23px]"
        transition:fly|local={{ x: 8 }}
      >
        <SendIcon
          class="w-full h-full text-red-400 hover:text-red-300 transition-colors"
        />
      </button>
    {/if}
  </form>
</div>

<style lang="postcss">
  .message-group {
    @apply flex flex-col items-start space-y-0.5 max-w-[80%];
  }

  .message-group.from-me {
    @apply ml-auto items-end;
  }

  .message-group.from-me > aside {
    @apply hidden;
  }

  .chat {
    @apply px-2.5 py-1.5 text-sm font-mono rounded-md max-w-full break-words;
    @apply bg-zinc-800/80 hover:bg-zinc-700/80 transition-colors;
  }

  .message-group.from-me .chat {
    @apply bg-red-900/60 hover:bg-red-800/60;
  }
</style>
