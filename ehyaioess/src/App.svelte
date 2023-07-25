<script lang="ts">
  import TabLayout from "./lib/TabLayout.svelte";
  import FilePicker from "./lib/FilePicker.svelte";
  import ConversationPanel from "./lib/ConversationPanel.svelte";
    import FlexTest from "./lib/FlexTest.svelte";
</script>

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
    {
      name:"FlexTest",
      component: FlexTest,
    }
  ]}
  initial={2}
/>