<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import { Checkbox, Radio } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import {
        fillSendStore,
        sGetChoice,
        setQuestionValue,
    } from "../../../data/contexts/fill";
    import type { APIQuestionConfigurationOneOf2 } from "@paltiverse/palform-typescript-openapi";
    import QfClearButton from "./QFClearButton.svelte";

    const dispatch = createEventDispatcher<{ change: undefined }>();

    export let id: string;
    export let config: APIQuestionConfigurationOneOf2;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue ? sGetChoice(currentValue) : { option: [] };

    $: onChoiceChange = (e: Event) => {
        if (currentValue === undefined) return;

        if (config.choice.multi) {
            const t = e.target as HTMLInputElement;
            let newVal: string[];
            if (t.checked) {
                newVal = [...value.option, t.value];
            } else {
                newVal = value.option.filter((e) => e !== t.value);
            }

            setQuestionValue(id, {
                Choice: {
                    option: newVal,
                },
            });
        } else {
            setQuestionValue(id, {
                Choice: {
                    option: value.option,
                },
            });
        }
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
        <div class="rounded-lg border border-slate-300 dark:border-slate-600">
            {#if config.choice.multi}
                <Checkbox
                    on:change={onChoiceChange}
                    name={id}
                    value={option}
                    checked={value.option.includes(option)}
                    disabled={$fillSendStore?.loading}
                    class="p-4"
                >
                    {option}
                </Checkbox>
            {:else}
                <Radio
                    bind:group={value.option[0]}
                    on:change={onChoiceChange}
                    value={option}
                    name={id}
                    disabled={$fillSendStore?.loading}
                    class="p-4"
                >
                    {option}
                </Radio>
            {/if}
        </div>
    {/each}

    {#if value.option.length > 0}
        <QfClearButton on:click={onClear} disabled={$fillSendStore?.loading} />
    {/if}
</ol>
