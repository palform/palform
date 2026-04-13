<script lang="ts">
    import { Button, Spinner, type ButtonProps } from "flowbite-svelte";
    import type { Snippet } from "svelte";
    import type {
        HTMLButtonAttributes,
        MouseEventHandler,
    } from "svelte/elements";

    interface Props {
        loading?: boolean;
        disabled?: boolean;
        color?: "primary" | "red" | "light";
        type?: HTMLButtonAttributes["type"];
        size?: ButtonProps["size"];
        buttonClass?: string;
        title?: string | undefined;
        outline?: boolean;
        onclick?: MouseEventHandler<HTMLButtonElement>;
        children?: Snippet;
    }

    let {
        loading = false,
        disabled = false,
        color = "primary",
        type = "button",
        size = "md",
        buttonClass = "",
        title = undefined,
        outline = false,
        onclick,
        children,
    }: Props = $props();
</script>

<Button
    {onclick}
    {disabled}
    {color}
    {type}
    class={buttonClass}
    {outline}
    {size}
    {title}
>
    {#if loading}
        <Spinner class="me-4" size="4" color="primary" />
    {/if}
    {@render children?.()}
</Button>
