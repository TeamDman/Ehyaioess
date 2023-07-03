<script lang="ts">

  interface Tab {
    name: string;
    component: ConstructorOfATypedSvelteComponent;
  }

  export let tabs: Tab[] = [];
  let activeTab = 0;

  function setActiveTab(index: number) {
    activeTab = index;
  }
</script>

<div class="flex flex-col w-full h-full bg-red-400">
  <div>
    {#each tabs as tab, index}
      <button
        class={index === activeTab ? 'active' : ''}
        on:click={() => setActiveTab(index)}
      >
        {tab.name}
      </button>
    {/each}
  </div>
  <div class="grow bg-orange-900">
    {#each tabs as tab, index}
      {#if index === activeTab}
        <svelte:component this={tab.component} />
      {/if}
    {/each}
  </div>
</div>

<style>
  button {
    background-color: white;
    border: none;
    border-bottom: 2px solid gray;
    cursor: pointer;
    margin-right: 10px;
    padding: 5px 10px;
  }

  button.active {
    border-bottom: 2px solid black;
    font-weight: bold;
  }
</style>