<script lang="ts">
    interface Props {
        children?: import("svelte").Snippet;
        class?: string;
    }

    let { children, class: className }: Props = $props();
</script>

<div class={`w-full overflow-y-auto rounded-lg shadow-md ${className}`}>
    {@render children?.()}
</div>
