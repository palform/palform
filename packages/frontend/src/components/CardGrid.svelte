<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<div
    class={`lg:gap-8 gap-4 grid xl:grid-cols-3 lg:grid-cols-2 ${props.class}`}
>
    {@render props.children?.()}
</div>
