<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Alert, Button, Label, Modal, Select } from "flowbite-svelte";
    import InfoText from "../../../type/InfoText.svelte";
    import type {
        APIQuestionGroupStepStrategyJumpCase,
        APIQuestionGroupStepStrategyJumpCaseCondition,
    } from "@palform/palform-typescript-openapi";
    import QgStepStrategyNewCondition from "./QGStepStrategyNewCondition.svelte";
    import ConditionLabel from "./ConditionLabel.svelte";
    import { getFormCtx } from "../../../../data/contexts/orgLayout";
    import {
        getFormAdminContext,
        getGroupTitle,
    } from "../../../../data/contexts/formAdmin";

    interface Props {
        fromGroupId: string;
        class?: string;
        onsavenew: (newCase: APIQuestionGroupStepStrategyJumpCase) => void;
    }

    let { fromGroupId, class: className, onsavenew }: Props = $props();
    const formAdminCtx = getFormAdminContext();
    const formMetadataCtx = getFormCtx();

    let showCreateModal = $state(false);

    let targetGroupId: string = $state("");
    let binaryOperation: "And" | "Or" = $state("And");
    let conditions: APIQuestionGroupStepStrategyJumpCaseCondition[] = $state(
        []
    );

    const onNewCondition = (
        e: APIQuestionGroupStepStrategyJumpCaseCondition
    ) => {
        conditions = [...conditions, e];
    };

    const onDeleteCondition = (index: number) => {
        conditions.splice(index, 1);
        conditions = conditions;
    };

    let selectItems = $derived([
        ...$formAdminCtx.groups
            .filter((e) => e.id !== fromGroupId)
            .map((g) => ({
                name: getGroupTitle(
                    $formMetadataCtx.one_question_per_page,
                    $formAdminCtx,
                    g
                ),
                value: g.id,
            })),
        {
            name: "- (Submit form)",
            value: "SUBMIT",
        },
    ]);

    let valid = $derived(targetGroupId !== "");
    function onSaveClick() {
        if (!valid) return;

        onsavenew({
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
    }
</script>

<Button
    size="xs"
    color="light"
    class={className}
    onclick={() => (showCreateModal = true)}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add jump case
</Button>

<Modal outsideclose bind:open={showCreateModal} title="New jump case">
    <Label>
        Jump to {$formMetadataCtx.one_question_per_page
            ? "question"
            : "section"}
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
                        ondelete={() => onDeleteCondition(index)}
                    />
                {/each}
            </div>
        {/if}

        <QgStepStrategyNewCondition {fromGroupId} oncreate={onNewCondition} />
    </fieldset>

    {#if conditions.length === 0}
        <Alert>
            Because there are no conditions, this jump case will <strong
                >always</strong
            > match unless another case above it matches.
        </Alert>
    {/if}

    {#snippet footer()}
        <Button onclick={onSaveClick} disabled={!valid}>Save</Button>
    {/snippet}
</Modal>
