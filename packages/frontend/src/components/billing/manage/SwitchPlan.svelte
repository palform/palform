<script lang="ts">
    import type {
        APIBillingPlan,
        APIBillingPlanPrice,
        APIBillingSubscription,
    } from "@paltiverse/palform-typescript-openapi";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import PlanComparison from "../comparison/PlanComparison.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import SwitchPlanPreview from "./SwitchPlanPreview.svelte";

    export let subscription: APIBillingSubscription;
    const orgCtx = getOrgContext();

    let loading = false;
    let previewModalWithPriceId: APIBillingPlanPrice | undefined = undefined;
    let previewModalAnnual = false;

    let showPreviewModal = false;
    $: onSubscriptionSelect = async (
        e: CustomEvent<{
            plan: APIBillingPlan;
            annual: boolean;
            trial: boolean;
        }>
    ) => {
        const plan = e.detail.plan;
        const priceId = e.detail.annual
            ? plan.price_annually
            : plan.price_monthly;

        previewModalWithPriceId = priceId;
        previewModalAnnual = e.detail.annual;
        showPreviewModal = true;
    };

    $: onConfirmChange = async () => {
        if (previewModalWithPriceId === undefined) return;

        showPreviewModal = false;
        loading = true;
        try {
            await APIs.billingPlans().then((a) =>
                a.billingPlanSwitch($orgCtx.org.id, false, {
                    new_stripe_price_id:
                        previewModalWithPriceId!.stripe_price_id,
                })
            );
            await showSuccessToast("Updated your plan successfully! Enjoy :)");
            window.location.reload();
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<SectionHeading>Switch plan</SectionHeading>
<PlanComparison
    currentPriceId={subscription.stripe_plan_price_id}
    allowTrial={false}
    fixCurrency={subscription.currency}
    on:select={onSubscriptionSelect}
    disabled={loading}
/>

{#if previewModalWithPriceId}
    <SwitchPlanPreview
        {subscription}
        newPriceId={previewModalWithPriceId.stripe_price_id}
        newPriceAmount={previewModalWithPriceId.amount}
        newPriceAnnual={previewModalAnnual}
        bind:open={showPreviewModal}
        on:accept={onConfirmChange}
    />
{/if}
