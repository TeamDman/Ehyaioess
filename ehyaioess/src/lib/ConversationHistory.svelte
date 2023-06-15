<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ConversationModel } from "./models";

    async function load(): Promise<Record<string, ConversationModel[]>> {
        return await invoke("listConversations");
    }

    let conversations: Record<string, ConversationModel[]> = {};
    load().then((data) => {
        conversations = data;
    });
</script>

<div>
    <p>There are {Object.keys(conversations).length} conversations.</p>
</div>
