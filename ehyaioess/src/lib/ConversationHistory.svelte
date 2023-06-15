<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ConversationModel } from "./models";
    import { viewConversation } from "./state";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";

    async function load(): Promise<Record<string, ConversationModel>> {
        console.log("Loading conversations");
        return await invoke("list_conversations");
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
    
    onMount(async () => {
        console.log("listening for conversation title changes");
        const unlisten = await listen(
            "conversation_title_changed",
            (event: { payload: { id: string; new_title: string } }) => {
                console.log("Conversation title changed", event.payload);
                conversations[event.payload.id].title = event.payload.new_title;
                conversations = conversations;
            }
        );
        onDestroy(unlisten);
    });
</script>

<div>
    <p>There are {Object.keys(conversations).length} conversations.</p>

    <ul>
        {#each Object.keys(conversations) as id}
            <li>
                <button
                    class="hover:bg-slate-500"
                    on:click|preventDefault={() =>
                        viewConversation.set(conversations[id])}
                    >{conversations[id].title}</button
                >
            </li>
        {/each}
    </ul>

    <button on:click|preventDefault={(e) => newConversation()}
        >New conversation</button
    >
</div>
