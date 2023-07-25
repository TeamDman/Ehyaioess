<script>
    let bigContent = false;
    $: content = bigContent
        ? [
              "abc ".repeat(500),
              "abc ".repeat(500),
              "abc ".repeat(500),
              "abc ".repeat(500),
          ]
        : ["lorem ipsum"];
</script>

<div class="flex h-full">
    <div class="bg-green-500 flex-none p-2 w-48">
        Sidebar Content Here
        <input bind:checked={bigContent} type="checkbox" />
    </div>
    <div class="flex flex-col flex-1 overflow-hidden">
        <div class="bg-blue-500 flex-none p-2">Header Content Here</div>
        <div class="content bg-purple-500 flex-grow p-2 overflow-auto">
            <!-- Add your big lorem ipsum content here -->
            {#each content as paragraph}
                <p class="bg-purple-300 m-4 rounded-md p-2">{paragraph}</p>
            {/each}
        </div>
        <div class="bg-blue-500 flex-none p-2">Footer Content Here</div>
    </div>
</div>
