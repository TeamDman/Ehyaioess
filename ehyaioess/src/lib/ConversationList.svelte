<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ConversationModel } from "./models";
    import { viewConversation } from "./state";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";

    async function load(): Promise<Record<string, ConversationModel>> {
        console.log("Loading conversations");
        const rtn: typeof conversations = await invoke("list_conversations");
        console.log("Received conversation history", rtn);
        return rtn;
    }

    let conversations: Record<string, ConversationModel> = {};
    load().then((data) => {
        conversations = data;
    });

    async function newConversation() {
        console.log("New conversation");
        let resp: ConversationModel = await invoke("new_conversation");
        console.log(resp.id, resp.title);
        conversations[resp.id] = resp;
        conversations = conversations;
    }

    let unlisten: UnlistenFn;
    onMount(async () => {
        console.log("listening for conversation title changes");
        unlisten = await listen(
            "conversation_title_changed",
            (event: {
                payload: { conversation_id: string; new_title: string };
            }) => {
                console.log("Conversation title changed", event.payload);
                conversations[event.payload.conversation_id].title =
                    event.payload.new_title;
                conversations = conversations;
            }
        );
    });
    onDestroy(()=>unlisten?unlisten():null);
</script>

<nav
    class="flex flex-col justify-between w-40 h-full bg-gradient-to-r from-slate-800 to-blue-800"
>
    <p class="text-white text-lg font-bold mb-4 p-4">
        There are {Object.keys(conversations).length} conversations.
    </p>
    <div class="overflow-y-auto overflow-x-visible">
        <!-- <div> -->

        <ul class="p-2">
            {#each Object.keys(conversations) as id}
                {@const isActive = $viewConversation?.id === id}
                <li class="mb-2">
                    <button
                        class="w-full text-left py-2 px-3 rounded bg-gradient-to-r from-blue-500 to-cyan-500 text-white hover:from-blue-400 hover:to-cyan-400 active:from-blue-600 active:to-cyan-600"
                        class:active-conversation={isActive}
                        on:click|preventDefault={() =>
                            viewConversation.set(isActive ? null : conversations[id])}
                        >{conversations[id].title}</button
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
