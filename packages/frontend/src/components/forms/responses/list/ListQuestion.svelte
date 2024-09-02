<script lang="ts">
    import type { QuestionSubmission } from "@paltiverse/palform-client-js-extra-types/QuestionSubmission";
    import { getResponsesContext } from "../../../../data/contexts/results";
    import {
        sGetAddress,
        sGetChoice,
        sGetChoiceMatrix,
        sGetDateTime,
        sGetFileUpload,
        sGetHidden,
        sGetPhoneNumber,
        sGetScale,
        sGetSignature,
        sGetText,
    } from "../../../../data/contexts/fill";
    import {
        qIsAddress,
        qIsChoice,
        qIsChoiceMatrix,
        qIsDateTime,
        qIsFileUpload,
        qIsHidden,
        qIsPhoneNumber,
        qIsScale,
        qIsSignature,
        qIsText,
    } from "../../../../data/contexts/questionsEditing";
    import CardBox from "../../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../../cardBox/CardBoxTitle.svelte";
    import ListQuestionAddress from "./ListQuestionAddress.svelte";
    import ListQuestionFileUpload from "./ListQuestionFileUpload.svelte";
    import ListQuestionSignature from "./ListQuestionSignature.svelte";
    import TableContainer from "../../../tables/TableContainer.svelte";
    import {
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
    } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { labelForQuestionDate } from "../../../../data/util/time";

    export let questionSubmission: QuestionSubmission;
    const ctx = getResponsesContext();

    $: question = $ctx.questions.find(
        (e) => e.id === questionSubmission.question_id
    );
</script>

<CardBox>
    {#if question === undefined}
        <p class="text-red-600">Question not found!</p>
        <p class="text-sm">
            This question doesn't currently exist. It might have been deleted
            since this response was made.
        </p>
    {:else}
        <CardBoxTitle>
            {question.title}
        </CardBoxTitle>

        {#if qIsText(question.configuration)}
            <p class="dark:text-gray-400">
                {sGetText(questionSubmission.data).value}
            </p>
        {:else if qIsChoice(question.configuration)}
            <ul class="list-disc list-inside">
                {#each sGetChoice(questionSubmission.data).option as option}
                    <li class="dark:text-gray-400">{option}</li>
                {/each}
            </ul>
        {:else if qIsScale(question.configuration)}
            <p
                class="mt-2 inline-block px-4 py-3 rounded-lg bg-primary-200 dark:bg-primary-600 text-primary-900 dark:text-primary-100 text-xl"
            >
                {sGetScale(questionSubmission.data).value ?? "Unspecified"}
            </p>
        {:else if qIsAddress(question.configuration)}
            <ListQuestionAddress
                address={sGetAddress(questionSubmission.data).address}
                location={sGetAddress(questionSubmission.data).point}
            />
        {:else if qIsPhoneNumber(question.configuration)}
            <p class="dark:text-gray-400">
                <strong
                    >{sGetPhoneNumber(questionSubmission.data)
                        .calling_code}</strong
                >
                {sGetPhoneNumber(questionSubmission.data).number}
            </p>
        {:else if qIsFileUpload(question.configuration)}
            <ListQuestionFileUpload
                fileId={sGetFileUpload(questionSubmission.data).file_id}
                contentType={sGetFileUpload(questionSubmission.data)
                    .content_type}
            />
        {:else if qIsSignature(question.configuration)}
            <ListQuestionSignature
                freeform={sGetSignature(questionSubmission.data).freeform}
                initial={sGetSignature(questionSubmission.data).initial}
                fullName={sGetSignature(questionSubmission.data).full_name}
            />
        {:else if qIsChoiceMatrix(question.configuration)}
            <TableContainer class="mt-2">
                <Table divClass="" striped>
                    <TableBody>
                        {#each sGetChoiceMatrix(questionSubmission.data).options as [row, cols]}
                            <TableBodyRow>
                                <TableBodyCell>{row}</TableBodyCell>
                                <TableBodyCell class="font-semibold">
                                    {cols.join(", ")}
                                </TableBodyCell>
                            </TableBodyRow>
                        {/each}
                    </TableBody>
                </Table>
            </TableContainer>
        {:else if qIsDateTime(question.configuration)}
            <p class="dark:text-gray-400">
                {labelForQuestionDate(
                    question.configuration.date_time,
                    DateTime.fromISO(
                        sGetDateTime(questionSubmission.data).value
                    )
                )}
            </p>
        {:else if qIsHidden(question.configuration)}
            <p class="dark:text-gray-400">
                {sGetHidden(questionSubmission.data).value}
            </p>
        {/if}
    {/if}
</CardBox>
