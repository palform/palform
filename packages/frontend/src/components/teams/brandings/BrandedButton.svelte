<script lang="ts">
    import type { Snippet } from "svelte";
    import type {
        HTMLButtonAttributes,
        MouseEventHandler,
    } from "svelte/elements";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import Color from "colorjs.io";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { Spinner } from "flowbite-svelte";
    import { colorWithLightness } from "../../../data/util/color";

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

    let {
        backgroundColorOverride,
        hoverBackgroundColorOverride,
        ringColorOverride,
        textColorOverride,
    } = $derived.by(() => {
        if (!$ctx) {
            return {
                backgroundColorOverride: null,
                hoverBackgroundColorOverride: null,
                ringColorOverride: null,
                textColorOverride: null,
            };
        }

        const primaryColor = new Color($ctx.primary_color);
        if (isDark) {
            return {
                backgroundColorOverride: colorWithLightness(primaryColor, 30),
                hoverBackgroundColorOverride: colorWithLightness(
                    primaryColor,
                    25
                ),
                ringColorOverride: colorWithLightness(primaryColor, 20),
                textColorOverride: colorWithLightness(primaryColor, 80),
            };
        } else {
            return {
                backgroundColorOverride: colorWithLightness(primaryColor, 40),
                hoverBackgroundColorOverride: colorWithLightness(
                    primaryColor,
                    30
                ),
                ringColorOverride: colorWithLightness(primaryColor, 80),
                textColorOverride: colorWithLightness(primaryColor, 20),
            };
        }
    });

    let rounding = $derived(getRoundingAmountForBrand($ctx));
</script>

<button
    class={[
        `brandedButton text-center font-medium focus-within:ring-4 focus-within:outline-none inline-flex items-center justify-center px-5 py-2.5 text-sm`,
        $ctx && outline
            ? "bg-transparent border border-(--branded-background-color) text-(--branded-text-color) hover:bg-(--branded-background-color) hover:text-white"
            : null,
        $ctx && !outline
            ? "bg-(--branded-background-color) text-white hover:bg-(--branded-hover-color)"
            : null,
        !$ctx && outline
            ? "border border-primary-700 dark:border-primary-400 text-primary-700 dark:text-primary-400 hover:bg-primary-700 hover:text-white"
            : null,
        !$ctx && !outline
            ? "bg-primary-700 dark:bg-primary-600 text-white hover:bg-primary-800 dark:hover:bg-primary-700"
            : null,
        $ctx ? "focus-within:ring-(--branded-ring-color)" : null,
        !$ctx
            ? "focus-within:ring-primary-300 dark:focus-within:ring-primary-800"
            : null,
        className ?? "",
    ]}
    style:--branded-background-color={backgroundColorOverride}
    style:--branded-hover-color={hoverBackgroundColorOverride}
    style:--branded-ring-color={ringColorOverride}
    style:--branded-text-color={textColorOverride}
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
