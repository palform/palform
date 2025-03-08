<script lang="ts">
    import {
        Button,
        ButtonGroup,
        Helper,
        Input,
        Label,
        Toggle,
    } from "flowbite-svelte";
    import {
        deleteGroup,
        deleteQuestion,
        getEditorQuestion,
        getFormEditorCtx,
        qIsAddress,
        qIsChoice,
        qIsChoiceMatrix,
        qIsDateTime,
        qIsFileUpload,
        qIsHidden,
        qIsInfo,
        qIsMeta,
        qIsPhoneNumber,
        qIsScale,
        qIsSignature,
        qIsText,
        updateQuestion,
        moveQuestion,
    } from "../../../data/contexts/formEditor";
    import CardBox from "../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../cardBox/CardBoxTitle.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faArrowDown,
        faArrowUp,
        faTrash,
    } from "@fortawesome/free-solid-svg-icons";
    import QeText from "./QEText.svelte";
    import type { APIQuestionConfiguration } from "@paltiverse/palform-typescript-openapi";
    import { getFormCtx } from "../../../data/contexts/orgLayout";
    import LoadingButton from "../../LoadingButton.svelte";
    import QeChoice from "./QEChoice.svelte";
    import CardBoxSubtitle from "../../cardBox/CardBoxSubtitle.svelte";
    import QeScale from "./QEScale.svelte";
    import QeAddress from "./QEAddress.svelte";
    import QePhoneNumber from "./QEPhoneNumber.svelte";
    import MarkdownEditor from "../../markdown/MarkdownEditor.svelte";
    import MarkdownView from "../../markdown/MarkdownView.svelte";
    import QeFileUpload from "./QEFileUpload.svelte";
    import QeSignature from "./QESignature.svelte";
    import QeChoiceMatrix from "./QEChoiceMatrix.svelte";
    import QuestionTypeLabel from "./QuestionTypeLabel.svelte";
    import QeDateTime from "./QEDateTime.svelte";
    import QeHidden from "./QEHidden.svelte";
    import type { ArrayMoveDirection } from "../../../data/util/arraySwap";

    export let questionId: string;
    const question = getEditorQuestion(questionId);
    const formMetadataCtx = getFormCtx();
    const formEditorCtx = getFormEditorCtx();

    $: editing = $formEditorCtx.currentlyEditing === questionId;

    let questionTitle = $question?.title ?? "";
    let questionDescription = $question?.description;
    let questionInternalName = $question?.internal_name;
    let questionRequired = $question?.required ?? false;

    let questionConfiguration = $question?.configuration;
    const onConfigUpdate = (e: CustomEvent<APIQuestionConfiguration>) =>
        (questionConfiguration = e.detail);

    $: onEditClick = () => {
        $formEditorCtx.currentlyEditing = questionId;
    };

    $: onSaveClick = async () => {
        if ($question === undefined || questionConfiguration === undefined)
            return;
        updateQuestion(formEditorCtx, {
            ...$question,
            title: questionTitle,
            description: questionDescription,
            internal_name: questionInternalName,
            required: questionRequired,
            configuration: questionConfiguration,
        });
        $formEditorCtx.currentlyEditing = undefined;
    };

    $: onDeleteClick = async (e: Event) => {
        e.stopPropagation();
        if (!$question) return;

        const groupId = $question.group_id;
        deleteQuestion(formEditorCtx, groupId, questionId);

        if ($formMetadataCtx.one_question_per_page) {
            deleteGroup(formEditorCtx, groupId);
        }
    };

    $: questionIndex =
        $question &&
        $formEditorCtx.questions[$question.group_id].findIndex(
            (e) => e.id === $question.id
        );
    $: canMoveUp = questionIndex && questionIndex > 0;
    $: canMoveDown =
        $question &&
        questionIndex !==
            $formEditorCtx.questions[$question?.group_id].length - 1;

    $: onMoveClick = (direction: ArrayMoveDirection) => {
        if (!$question) return;
        moveQuestion(formEditorCtx, $question, direction);
    };
</script>

{#if $question}
    <CardBox
        element="button"
        class={`block w-full text-left ${editing ? "" : "hover:scale-[1.02] active:scale-[0.98] transition-transform"}`}
        on:click={onEditClick}
        disabled={editing}
    >
        {#if !editing}
            <div class="flex justify-between items-start">
                <CardBoxTitle>
                    {$question.title}
                    {#if $question.required}
                        <span class="text-red-600 dark:text-red-400">*</span>
                    {/if}
                </CardBoxTitle>

                <ButtonGroup>
                    <Button
                        color="light"
                        disabled={!!$formEditorCtx.currentlyEditing}
                        on:click={onDeleteClick}
                    >
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                    {#if !$formMetadataCtx.one_question_per_page}
                        <Button
                            color="light"
                            disabled={!canMoveUp ||
                                !!$formEditorCtx.currentlyEditing}
                            on:click={(e) => {
                                e.stopPropagation();
                                onMoveClick("up");
                            }}
                        >
                            <FontAwesomeIcon icon={faArrowUp} />
                        </Button>
                        <Button
                            color="light"
                            disabled={!canMoveDown ||
                                !!$formEditorCtx.currentlyEditing}
                            on:click={(e) => {
                                e.stopPropagation();
                                onMoveClick("down");
                            }}
                        >
                            <FontAwesomeIcon icon={faArrowDown} />
                        </Button>
                    {/if}
                </ButtonGroup>
            </div>
        {:else}
            <Label>
                Title
                <Input class="mt-2" bind:value={questionTitle} autofocus />
                <Helper class="mt-2 text-gray-500 dark:text-gray-400">
                    See <a
                        href="https://docs.palform.app/forms/questions"
                        class="underline">documentation</a
                    > on how to use recall
                </Helper>
            </Label>
        {/if}

        {#if !editing}
            {#if $question.description}
                <CardBoxSubtitle>
                    <MarkdownView value={$question.description} />
                </CardBoxSubtitle>
            {/if}
        {:else}
            <div class="my-2">
                {#if questionDescription === null || questionDescription === undefined}
                    <Button
                        size="xs"
                        outline
                        class="mt-2"
                        on:click={() => (questionDescription = "")}
                        disabled={$formEditorCtx.loading}
                    >
                        Add description
                    </Button>
                {:else}
                    <Label for="question-description">Description</Label>
                    <MarkdownEditor
                        bind:value={questionDescription}
                        disabled={$formEditorCtx.loading}
                        imageTeamId={$formMetadataCtx.team_id}
                        imageFormId={$formMetadataCtx.id}
                        class="flex-1 mt-2"
                        id="question-description"
                    />
                    <Button
                        title="Delete description"
                        on:click={() => (questionDescription = null)}
                        disabled={$formEditorCtx.loading}
                        class="mt-3"
                        outline
                        size="xs"
                    >
                        <FontAwesomeIcon icon={faTrash} class="me-2" />
                        Delete description
                    </Button>
                {/if}

                {#if questionInternalName === null || questionInternalName === undefined}
                    <Button
                        size="xs"
                        outline
                        class="mt-"
                        on:click={() => (questionInternalName = "")}
                        disabled={$formEditorCtx.loading}
                    >
                        Add internal name
                    </Button>
                {:else}
                    <Label class="mt-4">
                        Internal name
                        <ButtonGroup class="mt-2 flex">
                            <Input
                                disabled={$formEditorCtx.loading}
                                bind:value={questionInternalName}
                            />
                            <Button
                                disabled={$formEditorCtx.loading}
                                on:click={() => (questionInternalName = null)}
                            >
                                <FontAwesomeIcon icon={faTrash} class="me-2" />
                            </Button>
                        </ButtonGroup>
                        <Helper class="mt-2 text-gray-500 dark:text-gray-400">
                            Identifies this question in recalls, tables, and
                            exports
                        </Helper>
                    </Label>
                {/if}
            </div>
        {/if}

        {#if editing && !qIsInfo($question.configuration) && !qIsHidden($question.configuration)}
            <Toggle
                bind:checked={questionRequired}
                class="mt-4 mb-2"
                disabled={$formEditorCtx.loading}
            >
                Required
            </Toggle>
        {/if}

        {#if !editing}
            <QuestionTypeLabel configuration={$question.configuration} />
        {/if}

        {#if editing && !qIsMeta($question.configuration)}
            <div
                class="bg-slate-50/40 dark:bg-slate-800 border dark:border-slate-700 p-4 mt-4 mb-2 rounded-lg"
            >
                {#if qIsText($question.configuration)}
                    <QeText
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsChoice($question.configuration)}
                    <QeChoice
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsScale($question.configuration)}
                    <QeScale
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsAddress($question.configuration)}
                    <QeAddress
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsPhoneNumber($question.configuration)}
                    <QePhoneNumber />
                {:else if qIsFileUpload($question.configuration)}
                    <QeFileUpload
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsSignature($question.configuration)}
                    <QeSignature
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsChoiceMatrix($question.configuration)}
                    <QeChoiceMatrix
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsDateTime($question.configuration)}
                    <QeDateTime
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {:else if qIsHidden($question.configuration)}
                    <QeHidden
                        config={$question.configuration}
                        on:update={onConfigUpdate}
                    />
                {/if}
            </div>
        {/if}
        {#if editing}
            <LoadingButton
                buttonClass="mt-2"
                size="sm"
                on:click={onSaveClick}
                loading={$formEditorCtx.loading}
                disabled={$formEditorCtx.loading}
            >
                Save
            </LoadingButton>
        {/if}
    </CardBox>
{/if}
