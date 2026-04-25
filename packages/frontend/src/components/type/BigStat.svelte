<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<p
    class={`text-5xl font-display text-black dark:text-primary-100 tracking-tight ${props.class}`}
>
    {@render props.children?.()}
</p>
