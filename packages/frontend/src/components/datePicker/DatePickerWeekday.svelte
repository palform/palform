<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
    }

    let { children }: Props = $props();
</script>

<p
    class="text-center flex items-center justify-center bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200"
>
    {@render children?.()}
</p>
