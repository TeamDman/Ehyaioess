<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { viewConversation } from "./state";
    import { onDestroy } from "svelte";

    let editingTitle = false;
    let currentTitle = $viewConversation.title;
    let editingTitleValue = currentTitle;
    let unsub = viewConversation.subscribe((newConversation) => {
        editingTitleValue = newConversation.title;
        currentTitle = newConversation.title;
    });
    onDestroy(() => unsub());

    function init(el) {
        el.focus();
    }

    async function commitTitleChange(id: string, newTitle: string) {
        if (newTitle === $viewConversation.title) {
            return;
        }

        console.log(`Changing title for ${id} to ${newTitle}`);
        await invoke("set_conversation_title", {
            id,
            new_title: newTitle,
        });
    }

    $: commitTitleChange($viewConversation.id, editingTitleValue);
</script>

<div
    class="bg-gradient-to-r from-cyan-500 to-blue-500 text-white w-full h-full"
>
    {#if !editingTitle}
        <button
            class="hover:bg-gradient-to-r from-indigo-500"
            on:click={() => (editingTitle = true)}
        >
            {$viewConversation.title}
        </button>
    {:else}
        <form>
            <label for="title">Title</label>
            <input id="title"
                use:init
                type="text"
                class="text-black"
                bind:value={editingTitleValue}
                on:blur={() => setTimeout(() => (editingTitle = false), 1000)}
            />
            <button class="hidden" type="submit">Save</button>
        </form>
    {/if}
    <p>Conversation</p>
</div>
