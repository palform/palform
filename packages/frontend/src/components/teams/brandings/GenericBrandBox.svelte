<script lang="ts">
    import type { Snippet } from "svelte";
    import {
        getBrandCtx,
        getBrandIsNonNeutralBackground,
        getLightnessForBrandBorder,
        getPaddingAmountForBrand,
        getRoundingAmountForBrand,
        getShadowAlphaForBrandBorder,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { colorWithLightness } from "../../../data/util/color";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        backgroundColor?: string;
        errorState?: boolean;
        neutralBorder?: boolean;
        ignorePadding?: boolean;
        element?: "div" | "button";
        disabled?: boolean;
        class?: string;
        children?: Snippet;
        onclick?: MouseEventHandler<HTMLDivElement | HTMLButtonElement>;
    }

    let {
        backgroundColor = undefined,
        errorState = false,
        neutralBorder = false,
        ignorePadding = false,
        element = "div",
        disabled = false,
        class: className = "",
        children,
        onclick,
    }: Props = $props();

    const brandCtx = getBrandCtx();
    let borderColorOverride = $state<string | undefined>(undefined);
    let shadowColorOverride = $state<string | undefined>(undefined);
    let isNonNeutralBg = $derived(getBrandIsNonNeutralBackground($brandCtx));

    $effect(() => {
        if ($brandCtx === undefined || errorState || neutralBorder) {
            borderColorOverride = undefined;
            shadowColorOverride = undefined;
            return;
        }

        if (isDarkMode()) {
            borderColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                20,
            );
            shadowColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                10,
            );
        } else {
            const borderLightness = getLightnessForBrandBorder($brandCtx);
            borderColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                borderLightness ?? 50,
                borderLightness === undefined ? 0 : 1,
            );
            shadowColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                40,
                getShadowAlphaForBrandBorder($brandCtx),
            );
        }
    });
</script>

<svelte:element
    this={element}
    {onclick}
    {disabled}
    role={element === "button" ? "button" : ""}
    class={`border ${neutralBorder ? "border-slate-200 dark:border-slate-600" : ""} shadow-sm ${isNonNeutralBg ? "bg-slate-50/80 dark:bg-slate-900" : ""} ${className}`}
    style:background-color={backgroundColor}
    style:border-color={borderColorOverride}
    style:--tw-shadow-color={shadowColorOverride}
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
    style:padding={ignorePadding
        ? undefined
        : getPaddingAmountForBrand($brandCtx)}
>
    {@render children?.()}
</svelte:element>
