<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";
    import type { APIQuestionScaleIcon } from "@paltiverse/palform-typescript-openapi";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { resolveScaleIcon } from "../../../data/util/scaleList";
    import { colorWithLightness } from "../../../data/util/color";

    export let questionId: string;
    export let label: string;
    export let active: boolean;
    export let isFirst: boolean;
    export let isLast: boolean;
    export let icon: APIQuestionScaleIcon;
    export let disabled = false;
    const dispatch = createEventDispatcher<{ click: undefined }>();
    const brandCtx = getBrandCtx();

    const onInputChange = (
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) => {
        if (e.currentTarget.checked) {
            dispatch("click");
        }
    };

    $: id = `${questionId}-${label}`;

    let borderColorOverride: undefined | string;
    let activeColorOverride: undefined | string;
    let textColorOverride: undefined | string;
    let iconHoverColorOverride: undefined | string;
    $: (() => {
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
    })();
</script>

{#if icon === "Numeric"}
    <label
        for={id}
        class={`block text-center cursor-pointer py-2 px-1 flex-1 rounded-none first:rounded-l-lg last:rounded-r-lg border-r-0 border-t border-b border-l last:border-r dark:border-gray-600 hover:bg-gray-200/80 dark:hover:bg-slate-700/80 peer-focus:ring-2 ring-white ${active ? "text-white" : "dark:text-slate-300"}`}
        style:background-color={active
            ? activeColorOverride ?? "rgb(147, 14, 189)"
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
            on:change={onInputChange}
            class="sr-only peer"
            {disabled}
        />
        {label}
    </label>
{:else}
    <label
        for={id}
        class={`block cursor-pointer ${active ? "text-primary-600 !text-[var(--brand-color)]" : "text-primary-200 dark:text-primary-900/40 !text-[var(--brand-color)] dark:!text-[var(--brand-color)]"} hover:text-primary-500 hover:!text-[var(--brand-hover-color)] dark:hover:text-primary-700 dark:hover:!text-[var(--brand-hover-color)]`}
        style:--brand-color={textColorOverride}
        style:--brand-hover-color={iconHoverColorOverride}
    >
        <input
            type="radio"
            {id}
            name={questionId}
            on:change={onInputChange}
            class="sr-only peer"
            {disabled}
        />
        <FontAwesomeIcon icon={resolveScaleIcon(icon)} size="2x" {id} />
    </label>
{/if}
