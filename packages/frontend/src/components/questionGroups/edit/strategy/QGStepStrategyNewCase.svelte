<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Alert, Button, Label, Modal, Select } from "flowbite-svelte";
    import {
        getGroupTitle,
        getResponsesContext,
    } from "../../../../data/contexts/results";
    import InfoText from "../../../type/InfoText.svelte";
    import type {
        APIQuestionGroupStepStrategyJumpCase,
        APIQuestionGroupStepStrategyJumpCaseCondition,
    } from "@paltiverse/palform-typescript-openapi";
    import QgStepStrategyNewCondition from "./QGStepStrategyNewCondition.svelte";
    import ConditionLabel from "./ConditionLabel.svelte";
    import { createEventDispatcher } from "svelte";

    export let fromGroupId: string;
    const formCtx = getResponsesContext();

    let showCreateModal = false;

    let targetGroupId: string = "";
    let binaryOperation: "And" | "Or" = "And";
    let conditions: APIQuestionGroupStepStrategyJumpCaseCondition[] = [];

    const onNewCondition = (
        e: CustomEvent<APIQuestionGroupStepStrategyJumpCaseCondition>,
    ) => {
        conditions = [...conditions, e.detail];
    };

    const onDeleteCondition = (index: number) => {
        conditions.splice(index, 1);
        conditions = conditions;
    };

    const dispatch = createEventDispatcher<{
        saveNew: APIQuestionGroupStepStrategyJumpCase;
    }>();

    $: selectItems = [
        ...$formCtx.groups
            .filter((e) => e.id !== fromGroupId)
            .map((g) => ({
                name: getGroupTitle(g),
                value: g.id,
            })),
        {
            name: "- (Submit form)",
            value: "SUBMIT",
        },
    ];

    $: valid = targetGroupId !== "";
    $: onSaveClick = () => {
        if (!valid) return;

        dispatch("saveNew", {
            target_group_id: targetGroupId === "SUBMIT" ? null : targetGroupId,
            conditions:
                binaryOperation === "And"
                    ? {
                          And: conditions,
                      }
                    : {
                          Or: conditions,
                      },
        });
        showCreateModal = false;
        binaryOperation = "And";
        conditions = [];
    };
</script>

<Button
    size="xs"
    color="light"
    outline
    class={$$props.class ?? ""}
    on:click={() => (showCreateModal = true)}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add jump case
</Button>

<Modal outsideclose bind:open={showCreateModal} title="New jump case">
    <Label>
        Jump to section
        <Select class="mt-2" bind:value={targetGroupId} items={selectItems} />
    </Label>

    <InfoText class="my-4">If...</InfoText>

    <fieldset
        class="p-4 border border-gray-200 dark:border-gray-600 rounded-md shadow-sm"
    >
        <Select
            bind:value={binaryOperation}
            items={[
                { name: "All of", value: "And" },
                { name: "One of", value: "Or" },
            ]}
        />

        {#if conditions.length > 0}
            <div class="space-y-4 mt-4">
                {#each conditions as condition, index}
                    <ConditionLabel
                        {condition}
                        class="bg-gray-50 dark:bg-slate-700"
                        showDelete
                        on:delete={() => onDeleteCondition(index)}
                    />
                {/each}
            </div>
        {/if}

        <QgStepStrategyNewCondition {fromGroupId} on:create={onNewCondition} />
    </fieldset>

    {#if conditions.length === 0}
        <Alert>
            Because there are no conditions, this jump case will <strong
                >always</strong
            > match unless another case above it matches.
        </Alert>
    {/if}

    <svelte:fragment slot="footer">
        <Button on:click={onSaveClick} disabled={!valid}>Save</Button>
    </svelte:fragment>
</Modal>
