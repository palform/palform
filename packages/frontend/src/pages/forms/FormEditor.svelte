<script lang="ts">
    import { writable } from "svelte/store";
    import { Alert } from "flowbite-svelte";
    import CreateQuestionGroup from "../../components/questionGroups/edit/CreateQuestionGroup.svelte";
    import QuestionGroupEditor from "../../components/questionGroups/edit/QuestionGroupEditor.svelte";
    import FormQuestionLimitWarning from "../../components/forms/FormQuestionLimitWarning.svelte";
    import { getFormCtx } from "../../data/contexts/orgLayout";
    import { getFormAdminContext } from "../../data/contexts/formAdmin";
    import {
        setFormEditorContext,
        initialiseEditorContext,
        type FormEditorContext,
    } from "../../data/contexts/formEditor";
    import FormAutosave from "../../components/forms/FormAutosave.svelte";
    import { slide } from "svelte/transition";
    import { flip } from "svelte/animate";

    const formAdminCtx = getFormAdminContext();
    const formMetadataCtx = getFormCtx();

    let formEditorCtx = writable<FormEditorContext>({
        questions: {},
        groups: [],
        loading: false,
        dirty: false,
        currentlyEditing: undefined,
    });
    setFormEditorContext(formEditorCtx);
    initialiseEditorContext(formEditorCtx, $formAdminCtx);
</script>

{#if $formEditorCtx.groups.length === 0}
    <Alert border class="mb-4">
        {#if $formMetadataCtx.one_question_per_page}
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

        <CreateQuestionGroup beforeIndex={0} alertMode class="mt-2" />
    </Alert>
{:else}
    <div class="space-y-4 2xl:px-[20%]">
        <FormAutosave />
        <FormQuestionLimitWarning class="!mt-6 !mb-4" />

        {#each $formEditorCtx.groups as group, index (group.id)}
            <div
                transition:slide
                animate:flip={{ duration: 200 }}
                class="space-y-4"
            >
                <CreateQuestionGroup beforeIndex={index} />
                <QuestionGroupEditor groupId={group.id} />
            </div>
        {/each}

        <CreateQuestionGroup beforeIndex={$formAdminCtx.groups.length} />
    </div>
{/if}
