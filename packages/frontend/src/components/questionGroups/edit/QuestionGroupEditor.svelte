<script lang="ts">
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
    import { flip } from "svelte/animate";

    interface Props {
        groupId: string;
        onserverSync?: () => void;
    }

    let { groupId, onserverSync }: Props = $props();

    const formEditorCtx = getFormEditorCtx();
    const formMetadataCtx = getFormCtx();
    let questionsInGroup = $derived(getEditorQuestionsInGroup(groupId));
    let group = $derived(getEditorQuestionGroup(groupId));

    let onDelete = $derived(async () => {
        if ($questionsInGroup.length > 0) {
            await showFailureToast(
                "Please delete or move all the questions in the section first!"
            );
            return;
        }

        deleteGroup(formEditorCtx, groupId);
        await showSuccessToast("Section deleted");
    });
</script>

{#if $group !== undefined}
    <QgContainer group={$group} ondelete={onDelete}>
        <div class="space-y-4 mb-4">
            {#each $questionsInGroup as question, index (question.id)}
                <div
                    animate:flip={{
                        duration: $formMetadataCtx.one_question_per_page
                            ? 0
                            : 200,
                    }}
                    class="space-y-4"
                >
                    {#if !$formMetadataCtx.one_question_per_page}
                        <CreateQuestion
                            {groupId}
                            beforeIndex={index}
                            oncreate={onserverSync}
                        />
                    {/if}
                    <EditQuestion questionId={question.id} />
                </div>
            {/each}
        </div>

        {#if !$formMetadataCtx.one_question_per_page}
            <CreateQuestion
                {groupId}
                beforeIndex={$questionsInGroup.length}
                oncreate={onserverSync}
            />
        {/if}

        <QgStepStrategyConfig {groupId} class="mt-4" />
    </QgContainer>
{/if}
