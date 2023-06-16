<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Conversation, ConversationMessage } from "./models";
    import NoConversationPlaceholder from "./NoConversationPlaceholder.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";

    export let conversationId: string;
    let conversation: Conversation | null = null;
    let isEditingTitle = false;
    let editingTitleValue = "";
    $: if (conversationId) {
        invoke("get_conversation", { conversation_id: conversationId }).then(
            (data: typeof conversation) => {
                conversation = data;
                editingTitleValue = data.title;
            }
        );
    }

    const unlisten = listen(
        "conversation_message_added",
        (event: { payload: ConversationMessage }) => {
            if (event.payload.conversation_id === conversationId) {
                conversation.history.push(event.payload);
                conversation = conversation;
            }
        }
    );
    onDestroy(async () => (await unlisten)());

    function focusInit(el) {
        el.focus();
    }

    let userInput = "";
    async function submitMessage() {
        if (userInput.trim() === "") return;
        console.log("Submitting message", userInput);
        await invoke("new_user_message", {
            conversation_id: conversationId,
            content: userInput,
        });
        userInput = "";
        await invoke("generate_assistant_message", {
            conversation_id: conversationId,
        });
    }

    $: isUserTurn =
        conversation?.history[conversation.history.length - 1]?.author !==
        "user";
</script>

{#if conversation !== null}
    <div
        class="bg-gradient-to-r from-cyan-500 to-blue-500 text-white w-full h-full"
    >
        <div class="text-center">
            {#if !isEditingTitle}
                <button
                    class="hover:bg-gradient-to-r from-indigo-500"
                    on:click={() => (isEditingTitle = true)}
                >
                    {conversation.title}
                </button>
            {:else}
                <form
                    on:submit|preventDefault={() =>
                        invoke("set_conversation_title", {
                            conversation_id: conversationId,
                            new_title: editingTitleValue,
                        })}
                >
                    <label for="title">Title</label>
                    <input
                        id="title"
                        use:focusInit
                        type="text"
                        class="text-black"
                        bind:value={editingTitleValue}
                        on:blur={() => (isEditingTitle = false)}
                    />
                    <button class="invisible" type="submit">Save</button>
                </form>
            {/if}
        </div>

        <div>
            <ul>
                {#each conversation.history as message}
                    <li>{message.author} - {message.content}</li>
                {/each}
            </ul>
        </div>

        <div>
            <form class="row" on:submit|preventDefault={() => submitMessage()}>
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
{:else}
    <NoConversationPlaceholder />
{/if}
