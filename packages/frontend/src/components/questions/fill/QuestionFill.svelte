<script lang="ts">
    import type { APIQuestion } from "@palform/palform-typescript-openapi";
    import QfText from "./QFText.svelte";
    import QfChoice from "./QFChoice.svelte";
    import {
        saveFormFill,
        selectQuestion,
        selectQuestionValidationErrors,
        templateFillQuestionText,
    } from "../../../data/contexts/fill";
    import { Alert } from "flowbite-svelte";
    import {
        qIsAddress,
        qIsChoice,
        qIsChoiceMatrix,
        qIsDateTime,
        qIsFileUpload,
        qIsHidden,
        qIsPhoneNumber,
        qIsRank,
        qIsScale,
        qIsSignature,
        qIsText,
    } from "../../../data/contexts/formEditor";
    import CardBox from "../../cardBox/CardBox.svelte";
    import CardBoxSubtitle from "../../cardBox/CardBoxSubtitle.svelte";
    import BrandedSpan from "../../teams/brandings/BrandedSpan.svelte";
    import QfScale from "./QFScale.svelte";
    import QfAddress from "./QFAddress.svelte";
    import QfPhoneNumber from "./QFPhoneNumber.svelte";
    import MarkdownView from "../../markdown/MarkdownView.svelte";
    import QfFileUpload from "./QFFileUpload.svelte";
    import QfSignature from "./QFSignature.svelte";
    import QfChoiceMatrix from "./QFChoiceMatrix.svelte";
    import QfDateTime from "./QFDateTime.svelte";
    import QfHidden from "./QFHidden.svelte";
    import QFRank from "./QFRank.svelte";

    interface Props {
        question: APIQuestion;
        isSample?: boolean;
    }

    let { question, isSample = false }: Props = $props();
    const config = $derived(question.configuration);
    const id = $derived(question.id);
    let currentValue = $derived(isSample ? undefined : selectQuestion(id));
    let validationError = $derived(
        isSample ? undefined : selectQuestionValidationErrors(id)
    );

    let timeout: number | undefined;
    const onUpdate = () => {
        if (timeout !== undefined) clearTimeout(timeout);
        timeout = setTimeout(() => {
            saveFormFill();
        }, 200) as unknown as number;
    };
</script>

{#if !qIsHidden(config)}
    <CardBox errorState={$validationError !== undefined}>
        <fieldset>
            {#if $validationError}
                <Alert color="red" class="mb-4">
                    <p>{$validationError.error}</p>
                </Alert>
            {/if}

            <legend class="text-primary-800 dark:text-primary-300">
                <label for={id}>
                    <BrandedSpan sizeGroup="h2">
                        {templateFillQuestionText(question.title)}
                    </BrandedSpan>
                    {#if question.required}
                        <span class="text-red-500 text-sm align-top ms-1">
                            *
                        </span>
                    {/if}
                </label>
            </legend>
            {#if question.description}
                <CardBoxSubtitle class="mb-2 last:mb-0">
                    <MarkdownView
                        value={question.description}
                        imagesWithFillToken
                    />
                </CardBoxSubtitle>
            {:else}
                <div class="h-2"></div>
            {/if}

            {#if $currentValue !== undefined || isSample}
                {#if qIsText(config)}
                    <QfText
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsChoice(config)}
                    <QfChoice
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsScale(config)}
                    <QfScale
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsAddress(config)}
                    <QfAddress
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsPhoneNumber(config)}
                    <QfPhoneNumber
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsFileUpload(config)}
                    <QfFileUpload
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsSignature(config)}
                    <QfSignature
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsChoiceMatrix(config)}
                    <QfChoiceMatrix
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsDateTime(config)}
                    <QfDateTime
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {:else if qIsRank(config)}
                    <QFRank
                        {id}
                        {config}
                        currentValue={$currentValue}
                        onchange={onUpdate}
                    />
                {/if}
            {/if}
        </fieldset>
    </CardBox>
{:else}
    <QfHidden {id} {config} currentValue={$currentValue} onchange={onUpdate} />
{/if}
