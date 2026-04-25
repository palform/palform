<script lang="ts">
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        disabled?: boolean;
        children?: import("svelte").Snippet;
        onclick: MouseEventHandler<HTMLButtonElement>;
    }

    let { disabled = false, children, onclick }: Props = $props();
</script>

<button
    class="text-primary-700 disabled:text-gray-500 hover:underline"
    {disabled}
    {onclick}
>
    {@render children?.()}
</button>
