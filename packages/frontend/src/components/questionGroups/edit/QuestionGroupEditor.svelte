<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import {
        getEditorCtx,
        getEditorQuestionsInGroup,
    } from "../../../data/contexts/questionsEditing";
    import CreateQuestion from "../../questions/edit/CreateQuestion.svelte";
    import EditQuestion from "../../questions/edit/EditQuestion.svelte";
    import {
        ctxGetGroup,
        deleteGroup,
        getResponsesContext,
    } from "../../../data/contexts/results";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import QgContainer from "./QGContainer.svelte";
    import QgStepStrategyConfig from "./strategy/QGStepStrategyConfig.svelte";

    export let groupId: string;
    const dispatch = createEventDispatcher<{ serverSync: undefined }>();

    const editorContext = getEditorCtx();
    const orgCtx = getOrgContext();
    const respCtx = getResponsesContext();
    const formCtx = getFormCtx();
    $: questionsInGroup = getEditorQuestionsInGroup(groupId);
    $: group = ctxGetGroup(groupId);

    let deleteLoading = false;
    $: onDelete = async () => {
        if ($questionsInGroup.length > 0) {
            await showFailureToast(
                "Please delete or move all the questions in the section first!"
            );
            return;
        }

        $editorContext.loading = true;
        deleteLoading = true;
        await deleteGroup(respCtx, $orgCtx.org.id, groupId);
        await showSuccessToast("Section deleted");
        deleteLoading = false;
        $editorContext.loading = false;
    };
</script>

{#if $group !== undefined}
    <QgContainer group={$group} loading={deleteLoading} on:delete={onDelete}>
        <div class="space-y-4 mb-4">
            {#each $questionsInGroup as question, index (question.id)}
                {#if !$formCtx.one_question_per_page}
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
            {/each}
        </div>
        {#if !$formCtx.one_question_per_page}
            <CreateQuestion
                {groupId}
                beforeIndex={$questionsInGroup.length}
                on:create={() => dispatch("serverSync")}
            />
        {/if}

        <QgStepStrategyConfig {groupId} class="mt-4" />
    </QgContainer>
{/if}
