<script lang="ts">
    import {
        setEditorContext,
        setQuestionsInEditorContext,
        type EditorContext,
    } from "../../data/contexts/questionsEditing";
    import { getResponsesContext } from "../../data/contexts/results";
    import { writable } from "svelte/store";
    import { Alert } from "flowbite-svelte";
    import CreateQuestionGroup from "../../components/questionGroups/edit/CreateQuestionGroup.svelte";
    import QuestionGroupEditor from "../../components/questionGroups/edit/QuestionGroupEditor.svelte";
    import FormQuestionLimitWarning from "../../components/forms/FormQuestionLimitWarning.svelte";
    import { getFormCtx } from "../../data/contexts/orgLayout";

    const respCtx = getResponsesContext();
    const formCtx = getFormCtx();

    let editorContext = writable<EditorContext>({
        questions: {},
        loading: false,
        currentlyEditing: undefined,
    });
    setEditorContext(editorContext);
    setQuestionsInEditorContext(editorContext, $respCtx.questions);

    const onServerSync = () => {
        $respCtx.questions = Object.values($editorContext.questions).flat();
    };
</script>

{#if $respCtx.groups.length === 0}
    <Alert border class="mb-4">
        {#if $formCtx.one_question_per_page}
            <h2 class="text-lg">Create your first question</h2>
            <p>
                To start creating your Palform, you'll need to write some
                questions.
            </p>
            <p>
                This form is one-question-at-a-time, so each question is on its
                own page.
            </p>
        {:else}
            <h2 class="text-lg">Create your first section</h2>
            <p>
                To start creating your Palform, you'll need to write some
                questions.
            </p>
            <p>Each question lives inside a section: think of them as pages.</p>
        {/if}

        <CreateQuestionGroup
            beforeIndex={0}
            alertMode
            class="mt-2"
            on:create={onServerSync}
        />
    </Alert>
{:else}
    <div class="space-y-4 2xl:px-[20%]">
        <FormQuestionLimitWarning class="!mt-6 !mb-4" />

        {#each $respCtx.groups as group, index (group.id)}
            <CreateQuestionGroup beforeIndex={index} on:create={onServerSync} />
            <QuestionGroupEditor
                groupId={group.id}
                on:serverSync={onServerSync}
            />
        {/each}

        <CreateQuestionGroup
            beforeIndex={$respCtx.groups.length}
            on:create={onServerSync}
        />
    </div>
{/if}
