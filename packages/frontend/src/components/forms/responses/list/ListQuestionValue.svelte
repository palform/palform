<script lang="ts">
    import type { QuestionSubmission } from "@paltiverse/palform-client-js-extra-types/QuestionSubmission";
    import type { APIQuestion } from "@paltiverse/palform-typescript-openapi";
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
    import ListQuestionAddress from "./ListQuestionAddress.svelte";
    import ListQuestionFileUpload from "./ListQuestionFileUpload.svelte";
    import ListQuestionSignature from "./ListQuestionSignature.svelte";
    import TableContainer from "../../../tables/TableContainer.svelte";
    import {
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        Tooltip,
    } from "flowbite-svelte";
    import { labelForQuestionDate } from "../../../../data/util/time";
    import { DateTime } from "luxon";

    export let questionSubmission: QuestionSubmission;
    export let question: APIQuestion;
    export let compact: boolean;
</script>

{#if qIsText(question.configuration)}
    <p
        class={`dark:text-gray-400 ${compact ? "text-xs text-ellipsis line-clamp-2" : ""}`}
    >
        {sGetText(questionSubmission.data).value}
    </p>
{:else if qIsChoice(question.configuration)}
    {#if compact}
        <p class="dark:text-gray-400 text-xs text-ellipsis line-clamp-2">
            {sGetChoice(questionSubmission.data).option.join(", ")}
        </p>
    {:else}
        <ul class="list-disc list-inside">
            {#each sGetChoice(questionSubmission.data).option as option}
                <li class="dark:text-gray-400">{option}</li>
            {/each}
        </ul>
    {/if}
{:else if qIsScale(question.configuration)}
    {#if compact}
        <p class="dark:text-gray-400 font-mono">
            {sGetScale(questionSubmission.data).value ?? ""}
        </p>
    {:else}
        <p
            class="mt-2 inline-block px-4 py-3 rounded-lg bg-primary-200 dark:bg-primary-600 text-primary-900 dark:text-primary-100 text-xl"
        >
            {sGetScale(questionSubmission.data).value ?? "Unspecified"}
        </p>
    {/if}
{:else if qIsAddress(question.configuration)}
    <ListQuestionAddress
        address={sGetAddress(questionSubmission.data).address}
        location={sGetAddress(questionSubmission.data).point}
        {compact}
    />
{:else if qIsPhoneNumber(question.configuration)}
    <p class="dark:text-gray-400">
        <strong>{sGetPhoneNumber(questionSubmission.data).calling_code}</strong>
        {sGetPhoneNumber(questionSubmission.data).number}
    </p>
    <!-- svelte-ignore missing-declaration -->
{:else if qIsFileUpload(question.configuration)}
    <ListQuestionFileUpload
        fileId={sGetFileUpload(questionSubmission.data).file_id}
        contentType={sGetFileUpload(questionSubmission.data).content_type}
        class={compact ? "" : "mt-2"}
        {compact}
    />
{:else if qIsSignature(question.configuration)}
    <ListQuestionSignature
        freeform={sGetSignature(questionSubmission.data).freeform}
        initial={sGetSignature(questionSubmission.data).initial}
        fullName={sGetSignature(questionSubmission.data).full_name}
        {compact}
    />
{:else if qIsChoiceMatrix(question.configuration)}
    {#if compact}
        <p class="dark:text-gray-400 underline decoration-dashed cursor-help">
            Not rendered
        </p>
        <Tooltip placement="left">
            Table is too big to render. Please see the individual view.
        </Tooltip>
    {:else}
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
    {/if}
{:else if qIsDateTime(question.configuration)}
    <p class="dark:text-gray-400">
        {labelForQuestionDate(
            question.configuration.date_time,
            DateTime.fromISO(sGetDateTime(questionSubmission.data).value)
        )}
    </p>
{:else if qIsHidden(question.configuration)}
    <p class="dark:text-gray-400">
        {sGetHidden(questionSubmission.data).value}
    </p>
{/if}
