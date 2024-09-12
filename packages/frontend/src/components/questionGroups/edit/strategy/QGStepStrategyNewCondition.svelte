<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { Button, Label, Select } from "flowbite-svelte";
    import {
        qIsAddress,
        qIsChoice,
        qIsChoiceMatrix,
        qIsDateTime,
        qIsHidden,
        qIsPhoneNumber,
        qIsScale,
        qIsText,
    } from "../../../../data/contexts/formEditor";
    import StrategyConfigText from "./conditionTypes/StrategyConfigText.svelte";
    import { createEventDispatcher } from "svelte";
    import type {
        APIQuestionGroupStepStrategyJumpCaseCondition,
        APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    } from "@paltiverse/palform-typescript-openapi";
    import StrategyConfigChoice from "./conditionTypes/StrategyConfigChoice.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import StrategyConfigScale from "./conditionTypes/StrategyConfigScale.svelte";
    import StrategyConfigPhoneNumber from "./conditionTypes/StrategyConfigPhoneNumber.svelte";
    import StrategyConfigAddress from "./conditionTypes/StrategyConfigAddress.svelte";
    import StrategyConfigChoiceMatrix from "./conditionTypes/StrategyConfigChoiceMatrix.svelte";
    import StrategyConfigDateTime from "./conditionTypes/StrategyConfigDateTime.svelte";
    import StrategyConfigHidden from "./conditionTypes/StrategyConfigHidden.svelte";
    import { getFormAdminContext } from "../../../../data/contexts/formAdmin";

    export let fromGroupId: string;
    const formAdminCtx = getFormAdminContext();
    let isAdding = false;

    let questionId = "";
    $: question = $formAdminCtx.questions.find((e) => e.id === questionId);

    const dispatch = createEventDispatcher<{
        create: APIQuestionGroupStepStrategyJumpCaseCondition;
    }>();
    $: onSave = (
        e: CustomEvent<APIQuestionGroupStepStrategyJumpCaseConditionMatcher>
    ) => {
        if (!question) return;
        dispatch("create", {
            question_id: questionId,
            matcher: e.detail,
        });
        isAdding = false;
    };
</script>

<Button
    size="xs"
    color="light"
    class="mt-2"
    outline
    on:click={() => (isAdding = !isAdding)}
>
    {#if !isAdding}
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Add condition
    {:else}
        Cancel
    {/if}
</Button>

{#if isAdding}
    <form
        class="mt-2 border border-gray-200 dark:border-gray-600 p-4 rounded-md"
    >
        <Label>
            Question
            <Select
                class="mt-2"
                bind:value={questionId}
                items={$formAdminCtx.questions
                    .filter((e) => e.group_id === fromGroupId)
                    .map((q) => ({ name: q.title, value: q.id }))}
            />
        </Label>

        {#if question !== undefined}
            <fieldset class="mt-4">
                {#if qIsText(question.configuration)}
                    <StrategyConfigText on:save={onSave} />
                {:else if qIsChoice(question.configuration)}
                    <StrategyConfigChoice
                        configuration={question.configuration.choice}
                        on:save={onSave}
                    />
                {:else if qIsScale(question.configuration)}
                    <StrategyConfigScale
                        on:save={onSave}
                        configuration={question.configuration.scale}
                    />
                {:else if qIsPhoneNumber(question.configuration)}
                    <StrategyConfigPhoneNumber on:save={onSave} />
                {:else if qIsAddress(question.configuration)}
                    <StrategyConfigAddress
                        configuration={question.configuration.address}
                        on:save={onSave}
                    />
                {:else if qIsChoiceMatrix(question.configuration)}
                    <StrategyConfigChoiceMatrix
                        configuration={question.configuration.choice_matrix}
                        on:save={onSave}
                    />
                {:else if qIsDateTime(question.configuration)}
                    <StrategyConfigDateTime
                        configuration={question.configuration.date_time}
                        on:save={onSave}
                    />
                {:else if qIsHidden(question.configuration)}
                    <StrategyConfigHidden
                        configuration={question.configuration.hidden}
                        on:save={onSave}
                    />
                {/if}
            </fieldset>
        {/if}
    </form>
{/if}
