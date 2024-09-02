<script lang="ts">
    import { fade } from "svelte/transition";
    import {
        getBrandCtx,
        getLightnessForBrandBorder,
        getPaddingAmountForBrand,
        getRoundingAmountForBrand,
        getShadowAlphaForBrandBorder,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { colorWithLightness } from "../../../data/util/color";

    export let backgroundColor: string = "";
    export let errorState = false;
    export let fadeIn = false;
    export let fadeOut = false;
    export let neutralBorder = false;
    export let ignorePadding = false;

    const brandCtx = getBrandCtx();
    let borderColorOverride: string | undefined = undefined;
    let shadowColorOverride: string | undefined = undefined;
    $: (() => {
        if ($brandCtx === undefined || errorState || neutralBorder) {
            borderColorOverride = undefined;
            shadowColorOverride = undefined;
            return;
        }

        if (isDarkMode()) {
            borderColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                20
            );
            shadowColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                10
            );
        } else {
            const borderLightness = getLightnessForBrandBorder($brandCtx);
            borderColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                borderLightness ?? 50,
                borderLightness === undefined ? 0 : 1
            );
            shadowColorOverride = colorWithLightness(
                $brandCtx.accent_color ?? $brandCtx.primary_color,
                40,
                getShadowAlphaForBrandBorder($brandCtx)
            );
        }
    })();
</script>

<div
    class={`border ${neutralBorder ? "border-slate-200 dark:border-slate-600" : ""} shadow-sm ${$$props.class ?? ""}`}
    style:background-color={backgroundColor}
    style:border-color={borderColorOverride}
    style:--tw-shadow-color={shadowColorOverride}
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
    style:padding={ignorePadding
        ? undefined
        : getPaddingAmountForBrand($brandCtx)}
    in:fade={{
        duration: fadeIn ? 100 : 0,
    }}
    out:fade={{
        duration: fadeOut ? 100 : 0,
    }}
>
    <slot />
</div>
