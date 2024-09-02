<script lang="ts">
    import type {
        APIBillingSubscription,
        APIBillingUpcomingInvoice,
    } from "@paltiverse/palform-typescript-openapi";
    import CardBoxSubtitle from "../../cardBox/CardBoxSubtitle.svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast } from "../../../data/toast";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import UpcomingInvoiceTable from "./UpcomingInvoiceTable.svelte";

    export let subscription: APIBillingSubscription;
    const orgCtx = getOrgContext();

    let loading = true;
    let upcomingInvoice: APIBillingUpcomingInvoice | undefined = undefined;
    APIs.billingInvoices()
        .then((a) =>
            a.billingInvoicePreview(
                $orgCtx.org.id,
                subscription.stripe_subscription_id
            )
        )
        .then((resp) => {
            upcomingInvoice = resp.data;
        })
        .catch((e) => showFailureToast(e))
        .finally(() => {
            loading = false;
        });
</script>

{#if loading}
    <SkeletonPrimitive height="300px" className="mt-4 mb-8 p-4">
        <SkeletonPrimitive height="30px" className="mb-4" />
        <SkeletonPrimitive height="220px" className="space-y-4 p-4">
            <SkeletonPrimitive height="20px" />
            <SkeletonPrimitive height="40px" />
            <SkeletonPrimitive height="40px" />
            <SkeletonPrimitive height="40px" />
        </SkeletonPrimitive>
    </SkeletonPrimitive>
{/if}

{#if loading === false && upcomingInvoice}
    <CardBoxSubtitle class="mt-4">
        Upcoming invoice on <strong
            >{parseServerTime(subscription.period_end).toLocaleString(
                DateTime.DATE_MED
            )}</strong
        >
    </CardBoxSubtitle>

    <UpcomingInvoiceTable {upcomingInvoice} class="my-4" />
{/if}
