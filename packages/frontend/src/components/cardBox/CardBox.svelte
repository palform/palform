<script lang="ts">
    import type { Snippet } from "svelte";
    import GenericBrandBox from "../teams/brandings/GenericBrandBox.svelte";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        backgroundColor?: string;
        errorState?: boolean;
        element?: "div" | "button";
        disabled?: boolean;
        class?: string;
        children?: Snippet;
        onclick?: MouseEventHandler<HTMLDivElement | HTMLButtonElement>;
    }

    let {
        backgroundColor = undefined,
        errorState = false,
        element = "div",
        disabled = false,
        class: className,
        children,
        onclick,
    }: Props = $props();
</script>

<GenericBrandBox
    class={`${errorState ? "border-red-300 dark:border-red-700 shadow-red-200 dark:shadow-red-600" : "border-primary-200/60 dark:border-primary-800/60 shadow-xl shadow-primary-300/10 dark:shadow-primary-700/10"} ${className ?? ""}`}
    {backgroundColor}
    {element}
    {onclick}
    {disabled}
>
    {@render children?.()}
</GenericBrandBox>
