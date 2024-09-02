<script lang="ts">
    import type { APIBillingInvoice } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import TableContainer from "../../tables/TableContainer.svelte";
    import {
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import InvoiceTableRow from "./InvoiceTableRow.svelte";
    import InfoText from "../../type/InfoText.svelte";

    const orgCtx = getOrgContext();
    let invoices: APIBillingInvoice[] | undefined;
    let invoicesLoading = true;
    APIs.billingInvoices()
        .then((a) => a.billingInvoiceList($orgCtx.org.id))
        .then((resp) => {
            invoices = resp.data;
            invoicesLoading = false;
        });
</script>

{#if invoicesLoading}
    <div class="space-y-1">
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
    </div>
{:else if invoices !== undefined}
    {#if invoices.length === 0}
        <InfoText>No invoices yet</InfoText>
    {:else}
        <TableContainer>
            <Table>
                <TableHead>
                    <TableHeadCell>#</TableHeadCell>
                    <TableHeadCell>Date</TableHeadCell>
                    <TableHeadCell>Price</TableHeadCell>
                    <TableHeadCell>
                        <span class="sr-only">Actions</span>
                    </TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each invoices as invoice (invoice.id)}
                        <InvoiceTableRow {invoice} />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}
