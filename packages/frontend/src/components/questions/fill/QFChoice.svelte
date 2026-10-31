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
    import { generateFormListItemId } from "../../../data/util/elementIdGen";

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

<div class="space-y-2" role="radiogroup">
    {#each config.choice.options as option}
        <QfChoiceLabelButton
            questionId={id}
            {option}
            isActive={value.option.includes(option)}
            isMulti={config.choice.multi}
        >
            {#if config.choice.multi}
                <input
                    id={generateFormListItemId(id, option)}
                    name={id}
                    value={option}
                    type="checkbox"
                    class="sr-only peer"
                    bind:group={value.option}
                    disabled={$fillSendStore?.loading}
                    onchange={onChoiceChange}
                    tabindex={0}
                />
            {:else}
                <input
                    id={generateFormListItemId(id, option)}
                    name={id}
                    value={option}
                    type="radio"
                    class="sr-only peer"
                    bind:group={value.option[0]}
                    disabled={$fillSendStore?.loading}
                    onchange={onChoiceChange}
                    tabindex={0}
                />
            {/if}
        </QfChoiceLabelButton>
    {/each}
</div>

<QfClearButton
    onclick={onClear}
    disabled={$fillSendStore?.loading}
    hidden={value.option.length === 0}
    class="mt-2"
/>
