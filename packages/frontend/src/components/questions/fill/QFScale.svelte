<script lang="ts">
    import type { ConfigScale } from "@palform/palform-typescript-openapi";
    import { genScaleList } from "../../../data/util/scaleList";
    import {
        fillSendStore,
        sGetScale,
        setQuestionValue,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import QfClearButton from "./QFClearButton.svelte";
    import QfScaleButton from "./QFScaleButton.svelte";

    interface Props extends QuestionFillProps<ConfigScale> {}

    let { id, config, currentValue, onchange }: Props = $props();

    let value = $derived(currentValue ? sGetScale(currentValue) : { value: 7 });
    let setNumber = $derived((n: number) => {
        if (currentValue === undefined) return;
        setQuestionValue(id, {
            Scale: {
                value: n,
            },
        });
        onchange();
    });
    const onClear = () => {
        if (currentValue === undefined) return;
        setQuestionValue(id, {
            Scale: { value: null },
        });
        onchange();
    };
</script>

<div
    class={`w-full flex items-center ${config.scale.icon === "Numeric" ? "" : "gap-x-2"}`}
>
    {#each genScaleList(config.scale.min, config.scale.max) as num}
        <QfScaleButton
            questionId={id}
            label={num.toString()}
            active={value.value === null ? false : num <= value.value}
            onclick={() => setNumber(num)}
            isFirst={num === config.scale.min}
            isLast={num === config.scale.max}
            icon={config.scale.icon ?? "Numeric"}
            disabled={$fillSendStore?.loading}
        />
    {/each}
</div>
{#if config.scale.icon === "Numeric"}
    <div class="w-full flex justify-between mt-2">
        <p class="text-gray-600 dark:text-gray-400">
            {#if config.scale.min_label}
                {config.scale.min_label}
            {/if}
        </p>
        <p class="text-gray-600 dark:text-gray-400">
            {#if config.scale.max_label}
                {config.scale.max_label}
            {/if}
        </p>
    </div>
{/if}

{#if value.value !== null}
    <QfClearButton onclick={onClear} disabled={$fillSendStore?.loading} />
{/if}
