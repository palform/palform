<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import type { APIQuestionConfigurationOneOf3 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import { genScaleList } from "../../../data/util/scaleList";
    import {
        fillSendStore,
        sGetScale,
        setQuestionValue,
    } from "../../../data/contexts/fill";
    import QfClearButton from "./QFClearButton.svelte";
    import QfScaleButton from "./QFScaleButton.svelte";

    export let id: string;
    export let config: APIQuestionConfigurationOneOf3;
    export let currentValue: QuestionSubmissionData | undefined;

    const dispatch = createEventDispatcher<{ change: undefined }>();
    $: value = currentValue ? sGetScale(currentValue) : { value: 7 };
    $: setNumber = (n: number) => {
        if (currentValue === undefined) return;
        setQuestionValue(id, {
            Scale: {
                value: n,
            },
        });
        dispatch("change");
    };
    const onClear = () => {
        if (currentValue === undefined) return;
        setQuestionValue(id, {
            Scale: { value: null },
        });
        dispatch("change");
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
            on:click={() => setNumber(num)}
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
    <QfClearButton on:click={onClear} disabled={$fillSendStore?.loading} />
{/if}
