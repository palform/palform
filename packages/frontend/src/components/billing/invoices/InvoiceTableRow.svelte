<script lang="ts">
    import type { APIBillingInvoice } from "@paltiverse/palform-typescript-openapi";
    import { Button, TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import { formatCurrency } from "../../../data/billing/util";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
    import InvoiceStatusLabel from "./InvoiceStatusLabel.svelte";

    export let invoice: APIBillingInvoice;
    $: onOpenInvoiceClick = () => {
        if (!invoice.url) return;
        window.open(invoice.url);
    };
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
            <Button outline on:click={onOpenInvoiceClick}>
                <FontAwesomeIcon icon={faArrowUpRightFromSquare} />
            </Button>
        </TableBodyCell>
    {/if}
</TableBodyRow>
