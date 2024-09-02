<script lang="ts">
    import type { ButtonProps } from "flowbite-svelte/Button.svelte";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import Color from "colorjs.io";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { Spinner } from "flowbite-svelte";

    const ctx = getBrandCtx();
    export let type: ButtonProps["type"] = "button";
    export let outline: ButtonProps["outline"] = false;
    export let disabled: ButtonProps["disabled"] = false;
    export let loading = false;

    const isDark = isDarkMode();
    let backgroundColorOverride: string | undefined;
    let hoverBackgroundColorOverride: string | undefined;
    let ringColorOverride: string | undefined;
    $: (() => {
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
    })();

    $: rounding = getRoundingAmountForBrand($ctx);
</script>

<button
    class={`brandedButton text-center font-medium focus-within:ring-4 focus-within:outline-none inline-flex items-center justify-center px-5 py-2.5 text-sm ${backgroundColorOverride ? (outline ? "bg-transparent border border-[var(--branded-background-color)] text-[var(--branded-background-color)]" : "bg-[var(--branded-background-color)] text-white") : outline ? "border border-primary-700 dark:border-primary-400 text-primary-700 dark:text-primary-400" : "bg-primary-700 dark:bg-primary-600 text-white"} ${hoverBackgroundColorOverride ? (outline ? "hover:bg-[var(--branded-background-color)] hover:text-white" : "hover:bg-[var(--branded-hover-color)]") : outline ? "hover:bg-primary-700 hover:text-white" : "hover:bg-primary-800 dark:hover:bg-primary-700"} ${ringColorOverride ? "focus-within:ring-[var(--branded-ring-color)]" : "focus-within:ring-primary-300 dark:focus-within:ring-primary-800"}  ${$$props.class}`}
    style:--branded-background-color={backgroundColorOverride}
    style:--branded-hover-color={hoverBackgroundColorOverride}
    style:--branded-ring-color={ringColorOverride}
    style:border-radius={rounding}
    {type}
    {disabled}
    on:click
>
    {#if loading}
        <Spinner class="me-4" size={4} color="white" />
    {/if}
    <slot />
</button>
