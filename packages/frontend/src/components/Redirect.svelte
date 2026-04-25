<script lang="ts">
    import { onMount } from "svelte";
    import { navigate } from "../router";

    interface Props {
        href: string;
    }

    let { href }: Props = $props();

    onMount(() => {
        void navigate(href);
    });
</script>
