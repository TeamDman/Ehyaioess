<script lang="ts">
    import type { ConversationModel } from "./models";
    import { emit } from "@tauri-apps/api/event";
    export let conversation: ConversationModel;

    let editingTitle = false;
    let editingTitleValue = conversation.title;

    function init(el) {
        el.focus();
    }

    function changeTitle(newTitle: string) {
        console.log(`Changing title for ${conversation.id} to ${newTitle}`);
        emit("change_conversation_title", {
            id: conversation.id,
            newTitle,
        });
        editingTitle = false;
    }
</script>

<div
    class="bg-gradient-to-r from-cyan-500 to-blue-500 text-white w-full h-full"
>
    {#if !editingTitle}
        <button
            class="hover:bg-gradient-to-r from-indigo-500"
            on:click={() => (editingTitle = true)}
        >
            {conversation.title}
        </button>
    {:else}
        <form on:submit|preventDefault={() => changeTitle(editingTitleValue)}>
            <input
                use:init
                type="text"
                class="text-black"
                bind:value={editingTitleValue}
                on:blur={() => (editingTitle = false)}
            />
            <button type="submit">Save</button>
        </form>
    {/if}
    <p>Conversation</p>
</div>
