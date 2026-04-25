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
    } from "@palform/palform-typescript-openapi";
    import StrategyConfigChoice from "./conditionTypes/StrategyConfigChoice.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import StrategyConfigScale from "./conditionTypes/StrategyConfigScale.svelte";
    import StrategyConfigPhoneNumber from "./conditionTypes/StrategyConfigPhoneNumber.svelte";
    import StrategyConfigAddress from "./conditionTypes/StrategyConfigAddress.svelte";
    import StrategyConfigChoiceMatrix from "./conditionTypes/StrategyConfigChoiceMatrix.svelte";
    import StrategyConfigDateTime from "./conditionTypes/StrategyConfigDateTime.svelte";
    import StrategyConfigHidden from "./conditionTypes/StrategyConfigHidden.svelte";
    import { getFormAdminContext } from "../../../../data/contexts/formAdmin";

    interface Props {
        fromGroupId: string;
    }

    let { fromGroupId }: Props = $props();
    const formAdminCtx = getFormAdminContext();
    let isAdding = $state(false);

    let questionId = $state("");
    let question = $derived(
        $formAdminCtx.questions.find((e) => e.id === questionId)
    );

    const dispatch = createEventDispatcher<{
        create: APIQuestionGroupStepStrategyJumpCaseCondition;
    }>();
    let onSave = $derived(
        (e: APIQuestionGroupStepStrategyJumpCaseConditionMatcher) => {
            if (!question) return;
            dispatch("create", {
                question_id: questionId,
                matcher: e,
            });
            isAdding = false;
        }
    );
</script>

<Button
    size="xs"
    color="light"
    class="mt-2"
    outline
    onclick={() => (isAdding = !isAdding)}
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
                    <StrategyConfigText onsave={onSave} />
                {:else if qIsChoice(question.configuration)}
                    <StrategyConfigChoice
                        configuration={question.configuration.choice}
                        onsave={onSave}
                    />
                {:else if qIsScale(question.configuration)}
                    <StrategyConfigScale
                        onsave={onSave}
                        configuration={question.configuration.scale}
                    />
                {:else if qIsPhoneNumber(question.configuration)}
                    <StrategyConfigPhoneNumber onsave={onSave} />
                {:else if qIsAddress(question.configuration)}
                    <StrategyConfigAddress
                        configuration={question.configuration.address}
                        onsave={onSave}
                    />
                {:else if qIsChoiceMatrix(question.configuration)}
                    <StrategyConfigChoiceMatrix
                        configuration={question.configuration.choice_matrix}
                        onsave={onSave}
                    />
                {:else if qIsDateTime(question.configuration)}
                    <StrategyConfigDateTime
                        configuration={question.configuration.date_time}
                        onsave={onSave}
                    />
                {:else if qIsHidden(question.configuration)}
                    <StrategyConfigHidden
                        configuration={question.configuration.hidden}
                        onsave={onSave}
                    />
                {/if}
            </fieldset>
        {/if}
    </form>
{/if}
