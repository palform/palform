<script lang="ts">
    import type { APIBillingInvoice } from "@palform/palform-typescript-openapi";
    import { Button, TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
    import InvoiceStatusLabel from "./InvoiceStatusLabel.svelte";
    import { formatCurrency } from "@palform/palform-frontend-common";

    interface Props {
        invoice: APIBillingInvoice;
    }

    let { invoice }: Props = $props();
    let onOpenInvoiceClick = $derived(() => {
        if (!invoice.url) return;
        window.open(invoice.url);
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        {invoice.id}<br />
        <InvoiceStatusLabel status={invoice.status} class="mt-1" />
    </TableBodyCell>
    <TableBodyCell>
        {parseServerTime(invoice.created).toLocaleString(DateTime.DATETIME_MED)}
    </TableBodyCell>
    <TableBodyCell>
        {formatCurrency(invoice.currency, invoice.amount)}
    </TableBodyCell>
    {#if invoice.url}
        <TableBodyCell>
            <Button outline onclick={onOpenInvoiceClick}>
                <FontAwesomeIcon icon={faArrowUpRightFromSquare} />
            </Button>
        </TableBodyCell>
    {/if}
</TableBodyRow>
