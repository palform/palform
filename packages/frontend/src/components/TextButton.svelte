<script lang="ts">
    import type { Snippet } from "svelte";
    import type {
        HTMLAttributeAnchorTarget,
        MouseEventHandler,
    } from "svelte/elements";

    interface Props {
        disabled?: boolean;
        inline?: boolean;
        href?: string;
        target?: HTMLAttributeAnchorTarget;
        onclick?: MouseEventHandler<HTMLAnchorElement>;
        children?: Snippet;
        class?: string;
    }

    let {
        disabled = false,
        inline = false,
        href,
        target,
        onclick,
        children,
        class: className,
    }: Props = $props();
</script>

<a
    class={`${inline ? "inline-block" : "block"} text-sm hover:underline ${disabled ? "cursor-not-allowed text-gray-400 dark:text-gray-500" : "cursor-pointer text-primary-800 dark:text-primary-200"} ${className}`}
    {href}
    {target}
    {onclick}
>
    {@render children?.()}
</a>
