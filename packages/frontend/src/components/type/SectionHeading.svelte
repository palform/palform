<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<h2 class={`text-xl text-slate-800 dark:text-slate-300 ${props.class}`}>
    {@render props.children?.()}
</h2>
