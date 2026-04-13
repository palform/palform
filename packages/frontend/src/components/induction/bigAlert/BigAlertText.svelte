<script lang="ts">
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<p class={`text-lg ${props.class}`}>
    {@render props.children?.()}
</p>
