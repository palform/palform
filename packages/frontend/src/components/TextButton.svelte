<script lang="ts">
    import type { HTMLAttributeAnchorTarget } from "svelte/elements";

    export let disabled = false;
    export let href: string | undefined = undefined;
    export let target: HTMLAttributeAnchorTarget | undefined = undefined;
</script>

<a
    class={`block text-sm hover:underline ${disabled ? "cursor-not-allowed text-gray-400 dark:text-gray-500" : "cursor-pointer text-primary-800 dark:text-primary-200"} ${$$props.class}`}
    {href}
    {target}
    on:click
>
    <slot />
</a>
