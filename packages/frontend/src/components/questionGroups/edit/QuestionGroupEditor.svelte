<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import CreateQuestion from "../../questions/edit/CreateQuestion.svelte";
    import EditQuestion from "../../questions/edit/EditQuestion.svelte";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { getFormCtx } from "../../../data/contexts/orgLayout";
    import QgContainer from "./QGContainer.svelte";
    import QgStepStrategyConfig from "./strategy/QGStepStrategyConfig.svelte";
    import {
        deleteGroup,
        getEditorQuestionGroup,
        getEditorQuestionsInGroup,
        getFormEditorCtx,
    } from "../../../data/contexts/formEditor";
    import { slide } from "svelte/transition";

    export let groupId: string;
    const dispatch = createEventDispatcher<{ serverSync: undefined }>();

    const formEditorCtx = getFormEditorCtx();
    const formMetadataCtx = getFormCtx();
    $: questionsInGroup = getEditorQuestionsInGroup(groupId);
    $: group = getEditorQuestionGroup(groupId);

    $: onDelete = async () => {
        if ($questionsInGroup.length > 0) {
            await showFailureToast(
                "Please delete or move all the questions in the section first!"
            );
            return;
        }

        deleteGroup(formEditorCtx, groupId);
        await showSuccessToast("Section deleted");
    };
</script>

{#if $group !== undefined}
    <QgContainer group={$group} on:delete={onDelete}>
        <div class="space-y-4 mb-4">
            {#each $questionsInGroup as question, index (question.id)}
                <div transition:slide class="space-y-4">
                    {#if !$formMetadataCtx.one_question_per_page}
                        <CreateQuestion
                            {groupId}
                            beforeIndex={index}
                            on:create={(_) => dispatch("serverSync")}
                        />
                    {/if}
                    <EditQuestion
                        questionId={question.id}
                        on:serverSync={() => dispatch("serverSync")}
                    />
                </div>
            {/each}
        </div>

        {#if !$formMetadataCtx.one_question_per_page}
            <CreateQuestion
                {groupId}
                beforeIndex={$questionsInGroup.length}
                on:create={() => dispatch("serverSync")}
            />
        {/if}

        <QgStepStrategyConfig {groupId} class="mt-4" />
    </QgContainer>
{/if}
