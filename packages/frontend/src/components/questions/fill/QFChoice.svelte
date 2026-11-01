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
    import { Input } from "flowbite-svelte";
    import { t } from "../../../data/contexts/i18n";

    interface Props extends QuestionFillProps<ConfigChoice> {}

    let { id, config, currentValue, onchange }: Props = $props();
    let value = $derived(
        currentValue ? sGetChoice(currentValue) : { option: [], other: null }
    );
    // svelte-ignore state_referenced_locally
    let otherValue = $state(value.other ?? "");
    // svelte-ignore state_referenced_locally
    let otherSelected = $state(!!value.other);

    let onChoiceChange = $derived((selectedOption: string) => {
        if (currentValue === undefined) return;

        let newOption = value.option;
        if (!config.choice.multi && selectedOption === "__other__") {
            newOption = [];
        }
        if (!config.choice.multi && selectedOption !== "__other__") {
            otherSelected = false;
        }

        setQuestionValue(id, {
            Choice: {
                option: newOption,
                other: otherSelected ? otherValue : null,
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
                other: null,
            },
        });
        otherSelected = false;
        otherValue = "";
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
                    onchange={() => onChoiceChange(option)}
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
                    onchange={() => onChoiceChange(option)}
                    tabindex={0}
                />
            {/if}
        </QfChoiceLabelButton>
    {/each}

    {#if config.choice.show_other}
        <QfChoiceLabelButton
            questionId={id}
            option={t("choice_other")}
            isActive={otherSelected}
            isMulti={config.choice.multi}
            for={generateFormListItemId(id, "__other__")}
        >
            {#if config.choice.multi}
                <input
                    id={generateFormListItemId(id, "__other__")}
                    name={`${id}__other__`}
                    value={"__other__"}
                    type="checkbox"
                    class="sr-only peer"
                    bind:checked={otherSelected}
                    disabled={$fillSendStore?.loading}
                    onchange={() => onChoiceChange("__other__")}
                    tabindex={0}
                />
            {:else}
                <input
                    id={generateFormListItemId(id, "__other__")}
                    name={`${id}__other__`}
                    value={"__other__"}
                    type="radio"
                    class="sr-only peer"
                    bind:group={otherSelected}
                    disabled={$fillSendStore?.loading}
                    onchange={() => onChoiceChange("__other__")}
                    tabindex={0}
                />
            {/if}
        </QfChoiceLabelButton>
    {/if}
</div>

{#if config.choice.show_other && otherSelected}
    <Input
        class="mt-2"
        bind:value={otherValue}
        placeholder={`${t("choice_other")}...`}
        onchange={() => onChoiceChange("__other__")}
    />
{/if}

<QfClearButton
    onclick={onClear}
    disabled={$fillSendStore?.loading}
    hidden={value.option.length === 0 && !otherSelected}
    class="mt-2"
/>
