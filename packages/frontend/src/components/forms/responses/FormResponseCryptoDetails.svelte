<script lang="ts">
    import type { SubmissionCryptoDetailsResponse } from "@paltiverse/palform-typescript-openapi";
    import {
        Alert,
        Spinner,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { parseServerTime } from "../../../data/util/time";
    import { isKnownKey } from "../../../data/crypto/details";
    import TableContainer from "../../tables/TableContainer.svelte";
    import { showFailureToast } from "../../../data/toast";
    import { getFormAdminContext } from "../../../data/contexts/formAdmin";

    export let submissionId: string;

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();

    let details: SubmissionCryptoDetailsResponse | undefined;
    let loading = true;
    $: APIs.submissions()
        .then((a) =>
            a.submissionsCrypto(
                $orgCtx.org.id,
                $formAdminCtx.formId,
                submissionId
            )
        )
        .then((resp) => {
            details = resp.data;
            loading = false;
        })
        .catch((e) => {
            loading = false;
            return showFailureToast(e);
        });
</script>

{#if loading}
    <Spinner />
{/if}

{#if details !== undefined}
    {#if details.decrypting_keys.length === 0}
        <Alert>
            <p>
                There are no keys that can be used to decrypt this response
                anywhere in your organisation.
            </p>
            <p>Unfortunately, this response is probably unrecoverable.</p>
        </Alert>
    {:else}
        <p>These keys can be used to decrypt this submission:</p>

        <TableContainer>
            <Table>
                <TableHead>
                    <TableHeadCell>Fingerprint</TableHeadCell>
                    <TableHeadCell>Owner</TableHeadCell>
                    <TableHeadCell>Created</TableHeadCell>
                    <TableHeadCell>Expires</TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each details.decrypting_keys as key (isKnownKey(key) ? key.Known.id : key.Unknown)}
                        <TableBodyRow>
                            {#if isKnownKey(key)}
                                <TableBodyCell>
                                    {key.Known.key_fingerprint}
                                </TableBodyCell>
                                <TableBodyCell>
                                    {#if key.Known.user_display_name}
                                        <span class="font-medium block">
                                            {key.Known.user_display_name}
                                        </span>
                                    {/if}
                                    {key.Known.user_email}
                                </TableBodyCell>
                                <TableBodyCell>
                                    {parseServerTime(
                                        key.Known.created_at
                                    ).toLocaleString(DateTime.DATETIME_MED)}
                                </TableBodyCell>
                                <TableBodyCell>
                                    {parseServerTime(
                                        key.Known.expires_at
                                    ).toLocaleString(DateTime.DATETIME_MED)}
                                </TableBodyCell>
                            {:else}
                                <TableBodyCell>
                                    {key.Unknown}
                                </TableBodyCell>
                            {/if}
                        </TableBodyRow>
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}
