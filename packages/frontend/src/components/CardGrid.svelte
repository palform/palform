<script lang="ts">
    interface Props {
        children?: import("svelte").Snippet;
        class?: string;
    }

    let { children, class: className }: Props = $props();
</script>

<div class={`lg:gap-8 gap-4 grid xl:grid-cols-3 lg:grid-cols-2 ${className}`}>
    {@render children?.()}
</div>
