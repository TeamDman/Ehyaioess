<script lang="ts">
  import Conversation from "./lib/Conversation.svelte";
  import ConversationHistory from "./lib/ConversationHistory.svelte";
  import { viewConversation } from "./lib/state";
</script>

<main class="flex">
  <div id="left-panel" class="w-40 h-full min-h-screen bg-slate-800">
    <ConversationHistory />
  </div>
  <div id="main-panel" class="container bg-slate-950 text-white">
    {#if $viewConversation != null}
      <Conversation />
    {:else}
      <p>no conversation selected</p>
    {/if}
  </div>
</main>
