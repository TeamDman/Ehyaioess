<script lang="ts">
  import Conversation from "./lib/Conversation.svelte";
  import ConversationList from "./lib/ConversationList.svelte";
  import NoConversationPlaceholder from "./lib/NoConversationPlaceholder.svelte";
  let selectedConversationId: string | null = null;
</script>

<main class="flex h-screen overflow-hidden">
  <ConversationList on:select={(e) => (selectedConversationId = e.detail)} />
  <div id="main-panel" class="flex-grow h-full overflow-auto">
    {#if selectedConversationId !== null}
      <Conversation bind:conversationId={selectedConversationId} />
    {:else}
      <NoConversationPlaceholder />
    {/if}
  </div>
</main>
