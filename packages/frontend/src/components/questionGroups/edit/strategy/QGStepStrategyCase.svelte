<script lang="ts">
    import type { APIQuestionGroupStepStrategyJumpCase } from "@paltiverse/palform-typescript-openapi";
    import { extractConditionList } from "../../../../data/util/stepStrategyConditions";
    import ConditionLabel from "./ConditionLabel.svelte";
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faTrash } from "@fortawesome/free-solid-svg-icons";
    import { createEventDispatcher } from "svelte";
    import { getFormCtx } from "../../../../data/contexts/orgLayout";
    import {
        getFormAdminContext,
        getGroupTitle,
    } from "../../../../data/contexts/formAdmin";

    export let strategyCase: APIQuestionGroupStepStrategyJumpCase;
    const formAdminCtx = getFormAdminContext();
    const formMetadataCtx = getFormCtx();
    let targetGroup = $formAdminCtx.groups.find(
        (e) => e.id === strategyCase.target_group_id
    );

    const dispatch = createEventDispatcher<{ delete: undefined }>();
    $: conditionList = extractConditionList(strategyCase.conditions);
</script>

<div class="border dark:border-slate-600 shadow-sm rounded-md py-2 px-4">
    <p>
        {#if targetGroup !== undefined}
            <span class="text-sm text-gray-800 dark:text-gray-300">Jump to</span
            >
            <span class="dark:text-gray-100">
                {getGroupTitle(
                    $formMetadataCtx.one_question_per_page,
                    $formAdminCtx,
                    targetGroup
                )}
            </span>
        {:else}
            <span class="dark:text-gray-100">Submit form</span>
        {/if}
        <span class="text-sm text-gray-800 dark:text-gray-300">
            {#if conditionList.length > 0}
                if
            {/if}
        </span>
    </p>

    {#if conditionList.length > 0}
        <div class="mt-2 space-y-2">
            {#each conditionList as condition}
                <ConditionLabel
                    {condition}
                    class="border dark:border-slate-600"
                />
            {/each}
        </div>
    {/if}

    <Button
        class={conditionList.length === 0 ? "mt-2" : "mt-3"}
        size="xs"
        color="light"
        outline
        on:click={() => dispatch("delete")}
    >
        <FontAwesomeIcon icon={faTrash} class="me-2" />
        Delete case
    </Button>
</div>
