<script lang="ts">
    import type { APILegalAgreement } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import TableSkeleton from "../../tables/TableSkeleton.svelte";
    import TableContainer from "../../tables/TableContainer.svelte";
    import {
        Button,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import NewLegalAgreementModal from "./NewLegalAgreementModal.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faArrowUpRightFromSquare,
        faPlus,
    } from "@fortawesome/free-solid-svg-icons";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import DestructiveModal from "../../DestructiveModal.svelte";

    const orgCtx = getOrgContext();

    let agreements = $state<APILegalAgreement[]>([]);
    let agreementsLoading = $state(true);
    let agreementTypes = $state<Record<string, string>>({});
    let agreementTypesLoading = $state(true);
    $effect(() => {
        APIs.legalAgreements()
            .then((a) => a.legalAgreementsList($orgCtx.org.id))
            .then((resp) => {
                agreements = resp.data;
            })
            .catch(showFailureToast)
            .finally(() => {
                agreementsLoading = false;
            });

        APIs.legalAgreements()
            .then((a) => a.legalAgreementsListAgreementTypes($orgCtx.org.id))
            .then(
                (resp) => (agreementTypes = resp.data as Record<string, string>)
            )
            .catch(showFailureToast)
            .finally(() => (agreementTypesLoading = false));
    });

    let onNewAgreement = $derived((newAgreement: APILegalAgreement) => {
        agreements = [newAgreement, ...agreements];
    });

    let showNewModal = $state(false);
    let showDeleteModal = $state(false);
    let selectedDeletionTarget = $state<string | null>(null);
    let deleteLoading = $state(false);

    let onDelete = $derived(() => {
        if (selectedDeletionTarget === null) return;

        deleteLoading = true;
        APIs.legalAgreements()
            .then((a) =>
                a.legalAgreementsDelete(selectedDeletionTarget!, $orgCtx.org.id)
            )
            .then(() => {
                agreements = agreements.filter(
                    (e) => e.id !== selectedDeletionTarget
                );
                showDeleteModal = false;
                selectedDeletionTarget = null;
                showSuccessToast("Legal agreement deleted");
            })
            .catch(showFailureToast)
            .finally(() => (deleteLoading = false));
    });

    const onDeleteClick = (agreementId: string) => {
        selectedDeletionTarget = agreementId;
        showDeleteModal = true;
    };
</script>

{#if !agreementTypesLoading}
    <NewLegalAgreementModal
        bind:open={showNewModal}
        {agreementTypes}
        {onNewAgreement}
    />
{/if}

<DestructiveModal
    bind:show={showDeleteModal}
    targetName="this legal agreement"
    ondelete={onDelete}
    loading={deleteLoading}
/>

{#if agreementsLoading}
    <TableSkeleton />
{:else}
    {#if agreements.length === 0}
        <InfoText class="mt-2">
            To generate and sign a Data Processing Agreement, please click the
            button below.
        </InfoText>
    {/if}

    <Button
        class="mt-4"
        onclick={() => (showNewModal = true)}
        disabled={agreementTypesLoading}
        loading={agreementTypesLoading}
        outline
    >
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Create new agreement
    </Button>

    {#if agreements.length > 0}
        <TableContainer class="mt-4">
            <Table>
                <TableHead>
                    <TableHeadCell>Agreement type</TableHeadCell>
                    <TableHeadCell>Created</TableHeadCell>
                    <TableHeadCell>
                        <span class="sr-only">Actions</span>
                    </TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each agreements as agreement (agreement.id)}
                        <TableBodyRow>
                            <TableBodyCell>
                                <strong>
                                    {agreementTypes[agreement.agreement_type]}
                                </strong>
                                <br />
                            </TableBodyCell>
                            <TableBodyCell>
                                {parseServerTime(
                                    agreement.created_at
                                ).toLocaleString(DateTime.DATE_MED)}
                            </TableBodyCell>
                            <TableBodyCell>
                                <Button
                                    href={agreement.document_url}
                                    target="_blank"
                                    class="me-2"
                                >
                                    <FontAwesomeIcon
                                        icon={faArrowUpRightFromSquare}
                                        class="me-2"
                                    />
                                    View
                                </Button>
                                <Button
                                    outline
                                    color="red"
                                    onclick={() => onDeleteClick(agreement.id)}
                                >
                                    Delete
                                </Button>
                            </TableBodyCell>
                        </TableBodyRow>
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}
