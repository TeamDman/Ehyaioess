<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { viewConversation } from "./state";
    import { onDestroy } from "svelte";

    let editingTitle = false;
    let currentTitle = $viewConversation.title;
    let editingTitleValue = currentTitle;
    let unsub = viewConversation.subscribe((newConversation) => {
        if (newConversation === null) return;
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

    let userInput = "";
    async function submitMessage() {
        if (userInput.trim() === "") return;
        console.log("Submitting message", userInput);
        await invoke("new_message", {
            conversation_id: $viewConversation.id,
            content: userInput,
        });
        userInput = "";
    }
</script>

<div
    class="bg-gradient-to-r from-cyan-500 to-blue-500 text-white w-full h-full"
>
    <div class="text-center">
        {#if !editingTitle}
            <button
                class="hover:bg-gradient-to-r from-indigo-500"
                on:click={() => (editingTitle = true)}
            >
                {$viewConversation.title}
            </button>
        {:else}
            <form on:submit|preventDefault={() => (editingTitle = false)}>
                <label for="title">Title</label>
                <input
                    id="title"
                    use:init
                    type="text"
                    class="text-black"
                    bind:value={editingTitleValue}
                    on:blur={() => (editingTitle = false)}
                />
                <button class="invisible" type="submit">Save</button>
            </form>
        {/if}
    </div>

    <div>
        <ul>
            {#each $viewConversation.history as message}
                <li>{message.author} - {message.content}</li>
            {/each}
        </ul>
    </div>

    <div>
        <form class="row" on:submit|preventDefault={()=>submitMessage()}>
            <input
                class="text-black"
                id="greet-input"
                placeholder="Enter a name..."
                bind:value={userInput}
            />
            <button type="submit">Greet</button>
        </form>
    </div>
</div>
