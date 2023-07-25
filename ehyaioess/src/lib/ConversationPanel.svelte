<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Conversation from "./Conversation.svelte";
    import NoConversationPlaceholder from "./NoConversationPlaceholder.svelte";

    import ConversationList from "./ConversationList.svelte";
    let selectedConversationId: string | null = null;
</script>

<div id="conversation-panel" class="flex h-full bg-purple-200">
    <ConversationList on:select={(e) => (selectedConversationId = e.detail)} />
    {#if selectedConversationId !== null}
        <Conversation bind:conversationId={selectedConversationId} />
    {:else}
        <NoConversationPlaceholder />
    {/if}
</div>
