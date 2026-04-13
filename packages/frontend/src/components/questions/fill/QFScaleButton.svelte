<script lang="ts">
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";
    import type { APIQuestionScaleIcon } from "@palform/palform-typescript-openapi";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { resolveScaleIcon } from "../../../data/util/scaleList";
    import { colorWithLightness } from "../../../data/util/color";

    interface Props {
        questionId: string;
        label: string;
        active: boolean;
        isFirst: boolean;
        isLast: boolean;
        icon: APIQuestionScaleIcon;
        disabled?: boolean;
        onclick: () => void;
    }

    let {
        questionId,
        label,
        active,
        isFirst,
        isLast,
        icon,
        disabled = false,
        onclick,
    }: Props = $props();
    const brandCtx = getBrandCtx();

    const onInputChange = (
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) => {
        if (e.currentTarget.checked) {
            onclick();
        }
    };

    let id = $derived(`${questionId}-${label}`);

    let borderColorOverride: undefined | string = $state();
    let activeColorOverride: undefined | string = $state();
    let textColorOverride: undefined | string = $state();
    let iconHoverColorOverride: undefined | string = $state();
    $effect(() => {
        borderColorOverride = undefined;
        activeColorOverride = undefined;
        textColorOverride = undefined;
        iconHoverColorOverride = undefined;

        if (!$brandCtx) return;

        const dark = isDarkMode();
        if (icon === "Numeric") {
            if (dark) {
                borderColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    60
                );
                activeColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    30
                );
                textColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    active ? 110 : 60
                );
            } else {
                borderColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    50
                );
                activeColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    40
                );
                textColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    active ? 100 : 20
                );
            }
        } else {
            if (dark) {
                textColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    active ? 40 : 10
                );
                iconHoverColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    30
                );
            } else {
                textColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    active ? 40 : 85
                );
                iconHoverColorOverride = colorWithLightness(
                    $brandCtx.primary_color,
                    45
                );
            }
        }
    });
</script>

{#if icon === "Numeric"}
    <label
        for={id}
        class={`block text-center cursor-pointer py-2 px-1 flex-1 rounded-none first:rounded-l-lg last:rounded-r-lg border-r-0 border-t border-b border-l last:border-r border-gray-300 dark:border-gray-600 hover:bg-gray-200/80 dark:hover:bg-slate-700/80 peer-focus:ring-2 ring-white ${active ? "text-white" : "dark:text-slate-300"}`}
        style:background-color={active
            ? (activeColorOverride ?? "rgb(147, 14, 189)")
            : ""}
        style:border-color={borderColorOverride}
        style:color={textColorOverride ?? (active ? "white" : "")}
        style:border-top-left-radius={isFirst
            ? getRoundingAmountForBrand($brandCtx)
            : undefined}
        style:border-bottom-left-radius={isFirst
            ? getRoundingAmountForBrand($brandCtx)
            : undefined}
        style:border-top-right-radius={isLast
            ? getRoundingAmountForBrand($brandCtx)
            : undefined}
        style:border-bottom-right-radius={isLast
            ? getRoundingAmountForBrand($brandCtx)
            : undefined}
    >
        <input
            type="radio"
            {id}
            name={questionId}
            onchange={onInputChange}
            class="sr-only peer"
            {disabled}
        />
        {label}
    </label>
{:else}
    <label
        for={id}
        class={`block cursor-pointer ${active ? ($brandCtx === undefined ? "text-primary-600" : "text-(--brand-color)") : $brandCtx === undefined ? "text-primary-200 dark:text-primary-900/40" : "text-(--brand-color) dark:text-(--brand-color)"} ${$brandCtx === undefined ? " hover:text-primary-500 dark:hover:text-primary-700" : "hover:text-(--brand-hover-color)! dark:hover:text-(--brand-hover-color)!"}`}
        style:--brand-color={textColorOverride}
        style:--brand-hover-color={iconHoverColorOverride}
    >
        <input
            type="radio"
            {id}
            name={questionId}
            onchange={onInputChange}
            class="sr-only peer"
            {disabled}
        />
        <FontAwesomeIcon icon={resolveScaleIcon(icon)} size="2x" />
    </label>
{/if}
