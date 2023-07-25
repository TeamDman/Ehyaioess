<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, tick } from "svelte";
    import * as bindings from "./bindings";

    export let conversationId: string;
    let conversationTitle = "Loading...";
    let conversationMessages: bindings.ConversationMessageAddedEvent[] = [];

    let isEditingTitle = false;
    let editingTitleValue = "";
    $: if (conversationId) {
        bindings.getConversation(conversationId).then((data: any) => {
            console.log("got conversation debug info", data);
        });
        bindings.getConversationTitle(conversationId).then((data: string) => {
            console.log("got title", data);
            conversationTitle = data;
            editingTitleValue = data;
        });
        bindings
            .getConversationMessages(conversationId)
            .then((data: bindings.ConversationMessageAddedEvent[]) => {
                console.log("got msgs", data);
                conversationMessages = data;
            });
    }

    const unlisten1 = listen(
        "conversation_title_changed",
        (event: { payload: bindings.ConversationTitleChangedEventPayload }) => {
            console.log("title change", event);
            if (event.payload.conversation_id === conversationId)
                conversationTitle = event.payload.new_title;
        }
    );
    onDestroy(async () => (await unlisten1)());
    const unlisten2 = listen(
        "conversation_message_added",
        (event: { payload: bindings.ConversationMessageAddedEventPayload }) => {
            if (event.payload.conversation_id === conversationId) {
                console.log("msg added", event);
                conversationMessages.push(event.payload);
                conversationMessages = conversationMessages;
            }
        }
    );
    onDestroy(async () => (await unlisten2)());

    function focusInit(el) {
        el.focus();
    }

    let userInput = "";
    async function submitMessage() {
        if (userInput.trim() === "") return;
        console.log("Submitting message", userInput);
        await bindings.newConversationUserMessage(conversationId, userInput);
        userInput = "";
        await bindings.newConversationAssistantMessage(conversationId);
    }

    let messageListElem;

    $: if (messageListElem && conversationMessages.length > 0) {
        console.log("ticking", messageListElem);
        tick().then(() => {
            // messageListElem.lastElementChild.scrollIntoView({
            //     behavior: "smooth",
            // });
        });
    }
</script>

<div
    id="conversation"
    class="
        flex
        flex-col
        flex-1
        overflow-hidden
        bg-gradient-to-r
        from-cyan-500
        to-blue-500
        text-white
    "
>
    <div class="flex justify-center items-center w-full py-5">
        {#if !isEditingTitle}
            <button
                class="px-6 py-3 text-lg font-semibold bg-transparent bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-purple-500 hover:to-indigo-600 rounded-lg shadow-lg transition-all"
                on:click={() => (isEditingTitle = true)}
            >
                {conversationTitle}
            </button>
        {:else}
            <form
                class="flex flex-col space-y-3"
                on:submit|preventDefault={() =>
                    bindings.setConversationTitle(
                        conversationId,
                        editingTitleValue
                    )}
            >
                <label class="font-semibold" for="title">Title</label>
                <input
                    id="title"
                    use:focusInit
                    type="text"
                    class="px-4 py-2 bg-white text-black rounded-lg shadow-lg"
                    bind:value={editingTitleValue}
                    on:blur={() => (isEditingTitle = false)}
                />
                <button class="invisible" type="submit">Save</button>
            </form>
        {/if}
    </div>

    <div class="overflow-auto w-full" id="style-2">
        <!-- class="w-full px-6 py-3 space-y-2 bg-white text-black rounded-lg shadow-lg" -->
        <ul bind:this={messageListElem}>
            {#each conversationMessages as message}
                <li class="my-2 flex flex-col mr-1">
                    <p class="px-3" class:self-end={message.author === "User"}>
                        {message.author}
                    </p>
                    <div
                        class="max-w-md font-semibold bg-gradient-to-tr from-orange-500 to-purple-700 rounded-xl p-2"
                        class:self-end={message.author === "User"}
                    >
                        {message.content}
                    </div>
                </li>
            {/each}
        </ul>
    </div>

    <div class="flex justify-center items-center w-full py-5">
        <form
            class="flex space-x-4"
            on:submit|preventDefault={() => submitMessage()}
        >
            <input
                class="px-4 py-2 w-full bg-white text-black rounded-lg shadow-lg"
                id="greet-input"
                placeholder="Enter a name..."
                bind:value={userInput}
            />
            <button
                class="px-6 py-2 bg-gradient-to-r from-indigo-500 to-purple-600 text-white rounded-lg shadow-lg"
                type="submit">Greet</button
            >
        </form>
    </div>
</div>

<style>
    #style-2::-webkit-scrollbar-track {
        -webkit-box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
        border-radius: 10px;
    }
    #style-2::-webkit-scrollbar {
        width: 12px;
    }
    #style-2::-webkit-scrollbar-thumb {
        border-radius: 10px;
        -webkit-box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
        @apply bg-gradient-to-b from-red-400 to-purple-600;
    }
</style>
