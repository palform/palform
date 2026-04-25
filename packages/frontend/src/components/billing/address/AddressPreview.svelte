<script lang="ts">
    import type { APIBillingCustomerAddress } from "@palform/palform-typescript-openapi";
    interface Props {
        address: APIBillingCustomerAddress;
    }

    let { address }: Props = $props();
</script>

{#if address.line1}
    {address.line1}<br />
{/if}
{#if address.line2}
    {address.line2}<br />
{/if}
{#if address.city}
    {address.city}<br />
{/if}
{#if address.postal_code}
    {address.postal_code}<br />
{/if}
{#if address.state}
    {address.state}
{/if}
{#if address.country}
    {address.country}
{/if}
