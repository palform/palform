<script lang="ts">
    import type { APIBillingUpcomingInvoice } from "@paltiverse/palform-typescript-openapi";
    import TableContainer from "../../tables/TableContainer.svelte";
    import {
        Badge,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { formatCurrency } from "@paltiverse/palform-frontend-common";

    export let upcomingInvoice: APIBillingUpcomingInvoice;
</script>

<TableContainer class={$$props.class}>
    <Table divClass="" striped>
        <TableHead>
            <TableHeadCell>
                <span class="sr-only">Item name</span>
            </TableHeadCell>
            <TableHeadCell>Price</TableHeadCell>
            <TableHeadCell>Quantity</TableHeadCell>
            <TableHeadCell>
                <span class="sr-only"> Subtotal </span>
            </TableHeadCell>
        </TableHead>
        <TableBody>
            {#each upcomingInvoice.lines as line}
                <TableBodyRow>
                    <TableBodyCell>
                        <span>
                            {line.name}
                            {#if line.proration}
                                <Badge>Proration</Badge>
                            {/if}
                        </span>
                        {#if line.proration}
                            <span
                                class="block text-xs mt-1 text-gray-700 dark:text-gray-300"
                            >
                                {line.stripe_description}
                            </span>
                        {/if}
                    </TableBodyCell>
                    <TableBodyCell>
                        {formatCurrency(
                            upcomingInvoice.currency,
                            line.unit_price
                        )}
                        {#if line.unit_price_per !== 1}
                            <span class="block text-sm text-gray-600">
                                per {line.unit_price_per}
                            </span>
                        {/if}
                    </TableBodyCell>
                    <TableBodyCell>{line.quantity}</TableBodyCell>
                    <TableBodyCell>
                        {formatCurrency(
                            upcomingInvoice.currency,
                            line.total_price
                        )}
                    </TableBodyCell>
                </TableBodyRow>
            {/each}
            {#each upcomingInvoice.promotions as promotion}
                <TableBodyRow>
                    <TableBodyCell class="italic">
                        {promotion.name}
                    </TableBodyCell>
                    <TableBodyCell />
                    <TableBodyCell />
                    <TableBodyCell>
                        {#if promotion.amount_off}
                            {formatCurrency(
                                upcomingInvoice.currency,
                                promotion.amount_off * -1
                            )}
                        {:else if promotion.percent_off}
                            -{promotion.percent_off}%
                        {/if}
                    </TableBodyCell>
                </TableBodyRow>
            {/each}
            {#if upcomingInvoice.tax_amount !== 0}
                <TableBodyRow>
                    <TableBodyCell>Subtotal</TableBodyCell>
                    <TableBodyCell />
                    <TableBodyCell />
                    <TableBodyCell>
                        {formatCurrency(
                            upcomingInvoice.currency,
                            upcomingInvoice.total_amount -
                                upcomingInvoice.tax_amount
                        )}
                    </TableBodyCell>
                </TableBodyRow>
                <TableBodyRow>
                    <TableBodyCell>Tax</TableBodyCell>
                    <TableBodyCell />
                    <TableBodyCell />
                    <TableBodyCell>
                        {formatCurrency(
                            upcomingInvoice.currency,
                            upcomingInvoice.tax_amount
                        )}
                    </TableBodyCell>
                </TableBodyRow>
            {/if}
            {#if upcomingInvoice.starting_balance !== 0}
                <TableBodyRow>
                    <TableBodyCell>Your balance contribution</TableBodyCell>
                    <TableBodyCell />
                    <TableBodyCell />
                    <TableBodyCell>
                        {formatCurrency(
                            upcomingInvoice.currency,
                            Math.min(
                                upcomingInvoice.starting_balance * -1,
                                upcomingInvoice.total_amount
                            ) * -1
                        )}
                    </TableBodyCell>
                </TableBodyRow>
            {/if}
            <TableBodyRow>
                <TableBodyCell class="font-bold">Total</TableBodyCell>
                <TableBodyCell />
                <TableBodyCell />
                <TableBodyCell class="font-bold">
                    {formatCurrency(
                        upcomingInvoice.currency,
                        upcomingInvoice.amount_due
                    )}
                </TableBodyCell>
            </TableBodyRow>
            {#if upcomingInvoice.ending_balance !== 0}
                <TableBodyRow>
                    <TableBodyCell>(new balance)</TableBodyCell>
                    <TableBodyCell />
                    <TableBodyCell />
                    <TableBodyCell>
                        {formatCurrency(
                            upcomingInvoice.currency,
                            upcomingInvoice.ending_balance * -1
                        )}
                    </TableBodyCell>
                </TableBodyRow>
            {/if}
        </TableBody>
    </Table>
</TableContainer>
