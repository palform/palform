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
    import type { IconProp } from "@fortawesome/fontawesome-svg-core";
    import type { Snippet } from "svelte";

    interface Props {
        questionId: string;
        option: string;
        isActive: boolean;
        isMulti: boolean;
        class?: string;
        icon?: IconProp;
        children?: Snippet;
    }

    let {
        questionId,
        option,
        isActive,
        isMulti,
        class: className,
        icon,
        children,
    }: Props = $props();

    const brandCtx = getBrandCtx();
    const isDark = isDarkMode();

    let { backgroundColorOverride, borderColorOverride } = $derived.by(() => {
        if ($brandCtx !== undefined && isActive) {
            return {
                backgroundColorOverride: colorWithLightness(
                    $brandCtx.primary_color,
                    isDark ? 15 : 90
                ),
                borderColorOverride: colorWithLightness(
                    $brandCtx.primary_color,
                    isDark ? 25 : 80
                ),
            };
        }
        return {
            backgroundColorOverride: undefined,
            borderColorOverride: undefined,
        };
    });

    const iconClass = "text-lg align-text-bottom text-gray-500 me-3";
</script>

<label
    for={`${questionId}-${option}`}
    class={`flex items-center border border-slate-200 dark:border-slate-800 text-gray-800 dark:text-gray-300 block p-4 text-sm cursor-pointer transition-colors bg-slate-50/50 dark:bg-slate-900 hover:bg-slate-50 dark:hover:bg-slate-800 active:bg-slate-100 dark:active:bg-slate-800/80 ${isActive && $brandCtx === undefined ? "bg-primary-200/60! dark:bg-primary-950!" : ""} ${className}`}
    style:font-size={`${getBaseREMFontSizeForBrand($brandCtx) * 0.85}rem`}
    style:border-radius={getRoundingAmountForBrand($brandCtx, true)}
    style:background-color={backgroundColorOverride}
    style:border-color={borderColorOverride}
>
    {#if icon}
        <FontAwesomeIcon {icon} class={iconClass} />
    {:else if isMulti}
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

    <span class="flex-1">
        {option}
    </span>

    {@render children?.()}
</label>
