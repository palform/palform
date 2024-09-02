<script lang="ts">
    import { Button, ButtonGroup, Input, Label, Toggle } from "flowbite-svelte";
    import {
        deleteQuestion,
        getEditorCtx,
        getEditorQuestion,
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
        saveQuestionsWithPositions,
    } from "../../../data/contexts/questionsEditing";
    import CardBox from "../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../cardBox/CardBoxTitle.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faArrowDown,
        faArrowUp,
        faEdit,
        faTrash,
    } from "@fortawesome/free-solid-svg-icons";
    import QeText from "./QEText.svelte";
    import type {
        APIQuestion,
        APIQuestionConfiguration,
    } from "@paltiverse/palform-typescript-openapi";
    import { APIs, humaniseAPIError } from "../../../data/common";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import { getResponsesContext } from "../../../data/contexts/results";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import LoadingButton from "../../LoadingButton.svelte";
    import QeChoice from "./QEChoice.svelte";
    import { createEventDispatcher } from "svelte";
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

    export let questionId: string;
    const question = getEditorQuestion(questionId);
    const orgCtx = getOrgContext();
    const formCtx = getResponsesContext();
    const formMetadataCtx = getFormCtx();
    const editorCtx = getEditorCtx();
    $: editing = $editorCtx.currentlyEditing === questionId;
    $: otherEditing = $editorCtx.currentlyEditing !== undefined && !editing;

    const dispatch = createEventDispatcher<{ serverSync: undefined }>();

    let questionTitle = $question?.title ?? "";
    let questionDescription = $question?.description;
    let questionRequired = $question?.required ?? false;

    $: onEditClick = () => {
        $editorCtx.currentlyEditing = questionId;
    };

    $: onQuestionUpdate = (q: APIQuestion) => {
        if (!$question) return;
        editorCtx.update((ctx) => {
            const i = ctx.questions[$question.group_id].findIndex(
                (e) => e.id === $question.id
            );
            ctx.questions[$question.group_id][i] = q;
            return ctx;
        });
    };

    const onConfigUpdate = (e: CustomEvent<APIQuestionConfiguration>) =>
        onQuestionUpdate({
            ...$question!,
            configuration: e.detail,
        });

    $: onSaveClick = async () => {
        if ($question === undefined) return;
        onQuestionUpdate({
            ...$question,
            title: questionTitle,
            description: questionDescription,
            required: questionRequired,
        });

        $editorCtx.loading = true;
        try {
            await APIs.questions().then((a) =>
                a.questionsSet(
                    $orgCtx.org.id,
                    $formCtx.formId,
                    $question.group_id,
                    questionId,
                    $question
                )
            );
            await showSuccessToast("Question saved");
            dispatch("serverSync");
        } catch (e) {
            await showFailureToast(humaniseAPIError(e));
        }

        $editorCtx.loading = false;
        $editorCtx.currentlyEditing = undefined;
    };

    $: onDeleteClick = async () => {
        if (!$question) return;
        $editorCtx.loading = true;
        try {
            await deleteQuestion(
                editorCtx,
                $orgCtx.org.id,
                $formCtx.formId,
                $question.group_id,
                questionId
            );
            await showSuccessToast("Question deleted");
            dispatch("serverSync");
        } catch (e) {
            await showFailureToast(humaniseAPIError(e));
        }
        $editorCtx.loading = false;
    };

    $: questionIndex =
        $question &&
        $editorCtx.questions[$question.group_id].findIndex(
            (e) => e.id === $question.id
        );
    $: canMoveUp = questionIndex && questionIndex > 0;
    $: canMoveDown =
        $question &&
        questionIndex !== $editorCtx.questions[$question?.group_id].length - 1;

    $: onMoveClick = async (direction: "up" | "down") => {
        if (!$question) return;
        editorCtx.update((ctx) => {
            const i = ctx.questions[$question.group_id].findIndex(
                (e) => e.id === $question.id
            );
            if (
                (direction === "up" && i === 0) ||
                (direction === "down" &&
                    i === ctx.questions[$question.group_id].length - 1)
            )
                return ctx;

            ctx.questions[$question.group_id].splice(i, 1);
            ctx.questions[$question.group_id].splice(
                direction === "up" ? i - 1 : i + 1,
                0,
                $question
            );
            return ctx;
        });
        $editorCtx.loading = true;
        await saveQuestionsWithPositions(
            editorCtx,
            $orgCtx.org.id,
            $formCtx.formId,
            $question.group_id
        );
        $editorCtx.loading = false;
        dispatch("serverSync");
    };
</script>

{#if $question}
    <CardBox>
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
                        disabled={!canMoveUp || !!$editorCtx.currentlyEditing}
                        on:click={() => onMoveClick("up")}
                    >
                        <FontAwesomeIcon icon={faArrowUp} />
                    </Button>
                    <Button
                        color="light"
                        disabled={!canMoveDown || !!$editorCtx.currentlyEditing}
                        on:click={() => onMoveClick("down")}
                    >
                        <FontAwesomeIcon icon={faArrowDown} />
                    </Button>
                </ButtonGroup>
            </div>
        {:else}
            <Label>
                Title
                <Input class="mt-2" bind:value={questionTitle} />
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
                        disabled={$editorCtx.loading}
                    >
                        Add description
                    </Button>
                {:else}
                    <Label for="question-description">Description</Label>
                    <MarkdownEditor
                        bind:value={questionDescription}
                        disabled={$editorCtx.loading}
                        imageTeamId={$formMetadataCtx.team_id}
                        imageFormId={$formMetadataCtx.id}
                        class="flex-1 mt-2"
                        id="question-description"
                    />
                    <Button
                        title="Delete description"
                        on:click={() => (questionDescription = null)}
                        disabled={$editorCtx.loading}
                        class="mt-3"
                        outline
                        size="xs"
                    >
                        <FontAwesomeIcon icon={faTrash} class="me-2" />
                        Delete description
                    </Button>
                {/if}
            </div>
        {/if}

        {#if editing && !qIsInfo($question.configuration) && !qIsHidden($question.configuration)}
            <Toggle
                bind:checked={questionRequired}
                class="mt-4 mb-2"
                disabled={$editorCtx.loading}
            >
                Required
            </Toggle>
        {/if}

        {#if !editing}
            <QuestionTypeLabel configuration={$question.configuration} />
        {/if}

        {#if !editing}
            <div class="mt-2">
                <Button
                    size="sm"
                    outline
                    on:click={onEditClick}
                    disabled={$editorCtx.loading || otherEditing}
                >
                    <FontAwesomeIcon icon={faEdit} class="me-2" />
                    Edit
                </Button>

                <Button
                    size="sm"
                    outline
                    on:click={onDeleteClick}
                    disabled={$editorCtx.loading || otherEditing}
                >
                    <FontAwesomeIcon icon={faTrash} class="me-2" />
                    Delete
                </Button>
            </div>
        {:else if !qIsMeta($question.configuration)}
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
                loading={$editorCtx.loading}
                disabled={$editorCtx.loading}
            >
                Save
            </LoadingButton>
        {/if}
    </CardBox>
{/if}
