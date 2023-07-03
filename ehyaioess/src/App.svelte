<script lang="ts">
  import TabLayout from "./lib/TabLayout.svelte";
  import FilePicker from "./lib/FilePicker.svelte";
  import ConversationPanel from "./lib/ConversationPanel.svelte";

</script>

<main class="flex h-screen overflow-hidden">
  <TabLayout
    tabs={[
      {
        name: "Conversations",
        component: ConversationPanel,
      },
      {
        name: "Settings",
        component: FilePicker,
      },
    ]}
  />
</main>
