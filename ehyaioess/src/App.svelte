<script lang="ts">
  import Conversation from "./lib/Conversation.svelte";
  import ConversationList from "./lib/ConversationList.svelte";
    import NoConversationPlaceholder from "./lib/NoConversationPlaceholder.svelte";
  import { viewConversation } from "./lib/state";

</script>

<main class="flex h-screen overflow-hidden">
  <ConversationList />
  <div id="main-panel" class="flex-grow h-full overflow-auto">
    {#if $viewConversation != null}
      <Conversation/>
    {:else}
      <NoConversationPlaceholder/>
    {/if}
  </div>
</main>