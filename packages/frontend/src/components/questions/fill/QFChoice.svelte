<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import { createEventDispatcher } from "svelte";
    import {
        fillSendStore,
        sGetChoice,
        setQuestionValue,
    } from "../../../data/contexts/fill";
    import type { APIQuestionConfigurationOneOf2 } from "@paltiverse/palform-typescript-openapi";
    import QfClearButton from "./QFClearButton.svelte";
    import QfChoiceLabelButton from "./QFChoiceLabelButton.svelte";

    const dispatch = createEventDispatcher<{ change: undefined }>();

    export let id: string;
    export let config: APIQuestionConfigurationOneOf2;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue ? sGetChoice(currentValue) : { option: [] };

    $: onChoiceChange = () => {
        if (currentValue === undefined) return;

        setQuestionValue(id, {
            Choice: {
                option: value.option,
            },
        });
        dispatch("change");
    };

    const onClear = async (e: Event) => {
        if (currentValue === undefined) return;

        e.preventDefault();
        setQuestionValue(id, {
            Choice: {
                option: [],
            },
        });
        dispatch("change");
    };
</script>

<ol class="space-y-2">
    {#each config.choice.options as option}
        {#if config.choice.multi}
            <input
                id={`${id}-${option}`}
                name={id}
                value={option}
                type="checkbox"
                class="hidden"
                bind:group={value.option}
                disabled={$fillSendStore?.loading}
                on:change={onChoiceChange}
            />
        {:else}
            <input
                id={`${id}-${option}`}
                name={id}
                value={option}
                type="radio"
                class="hidden"
                bind:group={value.option[0]}
                disabled={$fillSendStore?.loading}
                on:change={onChoiceChange}
            />
        {/if}

        <QfChoiceLabelButton
            questionId={id}
            {option}
            isActive={value.option.includes(option)}
            isMulti={config.choice.multi}
        />
    {/each}

    {#if value.option.length > 0}
        <QfClearButton on:click={onClear} disabled={$fillSendStore?.loading} />
    {/if}
</ol>
