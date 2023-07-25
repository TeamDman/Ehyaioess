<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { createEventDispatcher, onDestroy } from "svelte";
    import * as bindings from "./bindings";

    let conversationTitlesById: Record<string, string> = {};


    bindings.listConversationTitles().then(
        (data: typeof conversationTitlesById) => {
            conversationTitlesById = data;
        }
    );

    async function newConversation() {
        await bindings.newConversation();
    }

    const unlisten1 = listen(
        "conversation_title_changed",
        (event: {
            payload: { conversation_id: string; new_title: string };
        }) => {
            conversationTitlesById[event.payload.conversation_id] =
                event.payload.new_title;
            conversationTitlesById = conversationTitlesById;
        }
    );
    onDestroy(async () => (await unlisten1)());

    const unlisten2 = listen(
        "new_conversation",
        (event: {
            payload: { conversation_id: string; title: string };
        }) => {
            conversationTitlesById[event.payload.conversation_id] =
                event.payload.title;
            conversationTitlesById = conversationTitlesById;
        }
    );
    onDestroy(async () => (await unlisten2)());

    const dispatch = createEventDispatcher();
    function selectConversation(id: string) {
        selectedConversationId = selectedConversationId === id ? null : id;
        dispatch("select", selectedConversationId);
    }
    let selectedConversationId: string | null = null;
</script>

<nav
    id="conversation-list"
    class="flex flex-col justify-between w-40 h-full bg-gradient-to-r from-slate-800 to-blue-800"
>
    <p class="text-white text-lg font-bold mb-4 p-4">
        There are {Object.keys(conversationTitlesById).length} conversations.
    </p>
    <div class="overflow-y-auto overflow-x-visible">
        <!-- <div> -->

        <ul class="p-2">
            {#each Object.entries(conversationTitlesById) as [id, title]}
                {@const isActive = selectedConversationId === id}
                <li class="mb-2">
                    <button
                        class="w-full text-left py-2 px-3 rounded bg-gradient-to-r from-blue-500 to-cyan-500 text-white hover:from-blue-400 hover:to-cyan-400 active:from-blue-600 active:to-cyan-600"
                        class:active-conversation={isActive}
                        on:click|preventDefault={() => selectConversation(id)}
                        >{title}</button
                    >
                </li>
            {/each}
        </ul>
    </div>
    <div class="p-4">
        <button
            class="mt-4 w-full text-left py-2 px-3 rounded bg-gradient-to-r from-blue-500 to-cyan-500 text-white hover:from-blue-400 hover:to-cyan-400 active:from-blue-600 active:to-cyan-600"
            on:click|preventDefault={(e) => newConversation()}
            >New conversation</button
        >
    </div>
</nav>

<style>
    .active-conversation {
        @apply border-2 border-pink-500 rounded-lg bg-gradient-to-r from-pink-500 to-yellow-500 text-white shadow-lg;
    }
</style>
