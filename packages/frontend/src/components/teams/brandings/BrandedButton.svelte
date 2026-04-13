<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLButtonAttributes, MouseEventHandler } from "svelte/elements";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import Color from "colorjs.io";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { Spinner } from "flowbite-svelte";

    interface Props {
        type?: HTMLButtonAttributes["type"];
        outline?: boolean;
        disabled?: boolean;
        loading?: boolean;
        class?: string;
        children?: Snippet;
        onclick?: MouseEventHandler<HTMLButtonElement>;
    }

    let {
        type = "button",
        outline = false,
        disabled = false,
        loading = false,
        class: className,
        children,
        onclick,
    }: Props = $props();

    const ctx = getBrandCtx();
    const isDark = isDarkMode();
    let backgroundColorOverride = $state<string | undefined>(undefined);
    let hoverBackgroundColorOverride = $state<string | undefined>(undefined);
    let ringColorOverride = $state<string | undefined>(undefined);

    $effect(() => {
        backgroundColorOverride = undefined;
        hoverBackgroundColorOverride = undefined;
        ringColorOverride = undefined;
        if (!$ctx) return;

        const primaryColor = new Color($ctx.primary_color);
        if (isDark) {
            const bg = primaryColor.clone();
            bg.hsl.l = 30;
            backgroundColorOverride = bg.toString({ format: "hex" });
            const hv = primaryColor.clone();
            hv.hsl.l = 25;
            hoverBackgroundColorOverride = hv.toString({ format: "hex" });
            const rg = primaryColor.clone();
            rg.hsl.l = 20;
            ringColorOverride = rg.toString({ format: "hex" });
        } else {
            const bg = primaryColor.clone();
            bg.hsl.l = 40;
            backgroundColorOverride = bg.toString({ format: "hex" });
            const hv = primaryColor.clone();
            hv.hsl.l = 35;
            hoverBackgroundColorOverride = hv.toString({ format: "hex" });
            const rg = primaryColor.clone();
            rg.hsl.l = 80;
            ringColorOverride = rg.toString({ format: "hex" });
        }
    });

    let rounding = $derived(getRoundingAmountForBrand($ctx));
</script>

<button
    class={`brandedButton text-center font-medium focus-within:ring-4 focus-within:outline-none inline-flex items-center justify-center px-5 py-2.5 text-sm ${backgroundColorOverride ? (outline ? "bg-transparent border border-(--branded-background-color) text-(--branded-background-color)" : "bg-(--branded-background-color) text-white") : outline ? "border border-primary-700 dark:border-primary-400 text-primary-700 dark:text-primary-400" : "bg-primary-700 dark:bg-primary-600 text-white"} ${hoverBackgroundColorOverride ? (outline ? "hover:bg-(--branded-background-color) hover:text-white" : "hover:bg-(--branded-hover-color)") : outline ? "hover:bg-primary-700 hover:text-white" : "hover:bg-primary-800 dark:hover:bg-primary-700"} ${ringColorOverride ? "focus-within:ring-(--branded-ring-color)" : "focus-within:ring-primary-300 dark:focus-within:ring-primary-800"}  ${className ?? ""}`}
    style:--branded-background-color={backgroundColorOverride}
    style:--branded-hover-color={hoverBackgroundColorOverride}
    style:--branded-ring-color={ringColorOverride}
    style:border-radius={rounding}
    {type}
    {disabled}
    {onclick}
>
    {#if loading}
        <Spinner class="me-4" size="4" />
    {/if}
    {@render children?.()}
</button>
