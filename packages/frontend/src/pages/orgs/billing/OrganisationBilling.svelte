<script lang="ts">
    import type { APIBillingCustomer } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import SkeletonPrimitive from "../../../components/SkeletonPrimitive.svelte";
    import PlanManager from "../../../components/billing/PlanManager.svelte";

    const orgCtx = getOrgContext();
    let customerDetails: APIBillingCustomer | undefined;
    let detailsLoading = true;
    APIs.billingCustomers()
        .then((a) => a.billingCustomerGet($orgCtx.org.id))
        .then((resp) => {
            customerDetails = resp.data;
            detailsLoading = false;
        });
</script>

{#if detailsLoading}
    <div class="space-y-6">
        <SkeletonPrimitive height="300px" />
        <div class="grid lg:grid-cols-2 gap-10">
            <SkeletonPrimitive height="200px" />
            <SkeletonPrimitive height="200px" />
        </div>
    </div>
{:else if customerDetails}
    <PlanManager customer={customerDetails} />
{/if}
