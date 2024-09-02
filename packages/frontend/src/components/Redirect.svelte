<script lang="ts">
    import { onMount } from "svelte";
    import { navigate } from "svelte-routing";

    export let href: string;

    onMount(() => {
        navigate(href);
    });
</script>
