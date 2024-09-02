<script lang="ts">
    import { Alert, Button, Modal } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { DateTime } from "luxon";
    import { APIs } from "../../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../../data/toast";
    import { getResponsesContext } from "../../../../data/contexts/results";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import {
        submissionIsError,
        submissionIsSuccess,
    } from "../../../../data/crypto/results";
    import { navigateEvent } from "../../../../utils/navigate";
    import TextButton from "../../../TextButton.svelte";
    import FormResponseCryptoDetails from "../FormResponseCryptoDetails.svelte";
    import ListPaginator from "./ListPaginator.svelte";
    import ListGroup from "./ListGroup.svelte";
    import { isEntitled } from "../../../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../../../billing/entitlement/MissingEntitlementTooltip.svelte";
    import { parseServerTime } from "../../../../data/util/time";

    const respCtx = getResponsesContext();
    const orgCtx = getOrgContext();
    const cryptoDetailsEntitled = isEntitled("crypto_details");

    let selectedSubmissionIndex = 0;
    $: selectedSubmission = $respCtx.submissions[selectedSubmissionIndex];

    let deleteLoading = false;
    $: deleteSubmission = async (submissionId: string) => {
        deleteLoading = true;
        try {
            await APIs.submissions().then((a) =>
                a.submissionsDelete(
                    $orgCtx.org.id,
                    $respCtx.formId,
                    submissionId
                )
            );
            respCtx.update((ctx) => {
                return {
                    ...ctx,
                    submissions: ctx.submissions.filter(
                        (e) => e.id !== submissionId
                    ),
                };
            });
            selectedSubmissionIndex = 0;
            await showSuccessToast("Response deleted");
        } catch (e) {
            await showFailureToast(e);
        }

        deleteLoading = false;
    };

    $: fillToken =
        selectedSubmission &&
        $respCtx.tokens.find((e) => selectedSubmission.forToken === e.id);

    let showCryptoModal = false;
</script>

{#if $respCtx.submissions.length === 0}
    <Alert color="blue">No submissions yet.</Alert>
{:else}
    <ListPaginator bind:currentIndex={selectedSubmissionIndex} />
{/if}

{#if selectedSubmission !== undefined}
    <p class="text-sm text-gray-800 dark:text-gray-400">
        Submitted {parseServerTime(selectedSubmission.createdAt).toLocaleString(
            DateTime.DATETIME_MED
        )}
    </p>
    {#if fillToken !== undefined}
        <p class="text-sm text-gray-800 dark:text-gray-400">
            Using token <strong>{fillToken.nickname}</strong>
        </p>
    {/if}

    <div class="inline-flex items-center gap-2">
        <TextButton
            on:click={() => deleteSubmission(selectedSubmission.id)}
            disabled={deleteLoading}
        >
            Delete response
        </TextButton>
        <span class="text-primary-400">&bull;</span>
        <TextButton
            on:click={() => (showCryptoModal = true)}
            disabled={!$cryptoDetailsEntitled}
        >
            View encryption details
        </TextButton>
        <MissingEntitlementTooltip key="crypto_details" placement="bottom" />
    </div>

    <Modal title="Encryption details" outsideclose bind:open={showCryptoModal}>
        <FormResponseCryptoDetails submissionId={selectedSubmission.id} />
    </Modal>

    {#if submissionIsError(selectedSubmission)}
        <Alert color="red" border>
            <p class="font-bold">We can't decrypt this submission</p>
            <p>
                The key that was used to encrypt this submission is probably not
                available on your device.
            </p>
            <Button
                outline
                color="red"
                class="mt-2"
                href={`/orgs/${$orgCtx.org.id}/user/keys`}
                on:click={navigateEvent}
            >
                Manage keys
                <FontAwesomeIcon icon={faArrowRight} class="ml-2" />
            </Button>
            <p class="mt-4 text-xs">
                Full error: <code>{selectedSubmission.error}</code>
            </p>
        </Alert>
    {:else if submissionIsSuccess(selectedSubmission)}
        <ol class="space-y-6 mt-6">
            {#each selectedSubmission.groups as groupId (groupId)}
                <li>
                    <ListGroup {groupId} submission={selectedSubmission} />
                </li>
            {/each}
        </ol>
    {/if}
{/if}
