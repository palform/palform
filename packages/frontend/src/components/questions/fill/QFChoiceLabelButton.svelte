<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        getBaseREMFontSizeForBrand,
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { colorWithLightness } from "../../../data/util/color";
    import { isDarkMode } from "../../../data/util/darkMode";
    import {
        faSquareCheck as squareChecked,
        faCircleCheck as circleChecked,
    } from "@fortawesome/free-solid-svg-icons";
    import {
        faSquareCheck as squareEmpty,
        faCircleCheck as circleEmpty,
    } from "@fortawesome/free-regular-svg-icons";

    export let questionId: string;
    export let option: string;
    export let isActive: boolean;
    export let isMulti: boolean;

    const brandCtx = getBrandCtx();
    const isDark = isDarkMode();

    let backgroundColorOverride: string | undefined = undefined;
    let borderColorOverride: string | undefined = undefined;
    $: {
        if ($brandCtx !== undefined && isActive) {
            if (isActive) {
                backgroundColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    isDark ? 15 : 90
                );
                borderColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    isDark ? 25 : 80
                );
            }
        } else {
            backgroundColorOverride = undefined;
            borderColorOverride = undefined;
        }
    }

    const iconClass = "text-lg align-text-bottom text-gray-500 me-3";
</script>

<label
    for={`${questionId}-${option}`}
    class={`border border-slate-200 dark:border-slate-800 text-gray-800 dark:text-gray-300 block p-4 text-sm cursor-pointer transition-colors hover:bg-slate-50 dark:hover:bg-slate-800 active:bg-slate-100 dark:active:bg-slate-800/80 ${isActive && $brandCtx === undefined ? "!bg-primary-200/60 dark:!bg-primary-950" : ""}`}
    style:font-size={`${getBaseREMFontSizeForBrand($brandCtx) * 0.85}rem`}
    style:border-radius={getRoundingAmountForBrand($brandCtx, true)}
    style:background-color={backgroundColorOverride}
    style:border-color={borderColorOverride}
>
    {#if isMulti}
        {#if isActive}
            <FontAwesomeIcon icon={squareChecked} class={iconClass} />
        {:else}
            <FontAwesomeIcon icon={squareEmpty} class={iconClass} />
        {/if}
    {:else if isActive}
        <FontAwesomeIcon icon={circleChecked} class={iconClass} />
    {:else}
        <FontAwesomeIcon icon={circleEmpty} class={iconClass} />
    {/if}

    {option}
</label>
