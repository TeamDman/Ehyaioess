<script lang="ts">
    import * as bindings from "./bindings";
    // bindings.
    let files = [];
    bindings.listFiles().then(x => {
        console.log("got files",x);
        files = x
    });
</script>

<h1>hi there {files}</h1>