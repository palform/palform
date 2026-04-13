<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
    }

    let { children }: Props = $props();
</script>

<p class="text-gray-600 dark:text-gray-400">
    {@render children?.()}
</p>
