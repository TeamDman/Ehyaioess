<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Conversation from "./Conversation.svelte";
    import NoConversationPlaceholder from "./NoConversationPlaceholder.svelte";

    import ConversationList from "./ConversationList.svelte";
    let selectedConversationId: string | null = null;
</script>

<div class="flex flex-row bg-purple-200 h-full w-full">
    <ConversationList on:select={(e) => (selectedConversationId = e.detail)} />
    <div id="main-panel" class="grow">
        {#if selectedConversationId !== null}
            <Conversation bind:conversationId={selectedConversationId} />
        {:else}
            <NoConversationPlaceholder />
        {/if}
    </div>
</div>
