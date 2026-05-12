<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        children: Snippet;
    }
    let { children }: Props = $props();
</script>

<p class="text-gray-800 dark:text-gray-200 not-last:mb-2">
    {@render children()}
</p>
