<script lang="ts">
    import { Button, Spinner } from "flowbite-svelte";
    import type { ButtonProps } from "flowbite-svelte/Button.svelte";
    import type { HTMLButtonAttributes } from "svelte/elements";

    export let loading = false;
    export let disabled = false;
    export let color: "primary" | "red" | "light" = "primary";
    export let type: HTMLButtonAttributes["type"] = "button";
    export let size: ButtonProps["size"] = "md";
    export let buttonClass = "";
    export let title: string | undefined = undefined;
    export let outline: boolean = false;
</script>

<Button
    on:click
    {disabled}
    {color}
    {type}
    class={buttonClass}
    {outline}
    {size}
    {title}
>
    {#if loading}
        <Spinner class="me-4" size={4} color="white" />
    {/if}
    <slot />
</Button>
