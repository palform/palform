<script lang="ts">
    import {
        fillSendStore,
        sGetChoice,
        setQuestionValue,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import QfClearButton from "./QFClearButton.svelte";
    import QfChoiceLabelButton from "./QFChoiceLabelButton.svelte";
    import type { ConfigChoice } from "@palform/palform-typescript-openapi";

    interface Props extends QuestionFillProps<ConfigChoice> {}

    let { id, config, currentValue, onchange }: Props = $props();
    let value = $derived(
        currentValue ? sGetChoice(currentValue) : { option: [] }
    );

    let onChoiceChange = $derived(() => {
        if (currentValue === undefined) return;

        setQuestionValue(id, {
            Choice: {
                option: value.option,
            },
        });
        onchange();
    });

    const onClear = async (e: Event) => {
        if (currentValue === undefined) return;

        e.preventDefault();
        setQuestionValue(id, {
            Choice: {
                option: [],
            },
        });
        onchange();
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
                onchange={onChoiceChange}
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
                onchange={onChoiceChange}
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
        <QfClearButton onclick={onClear} disabled={$fillSendStore?.loading} />
    {/if}
</ol>
