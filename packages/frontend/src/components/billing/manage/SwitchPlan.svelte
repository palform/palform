<script lang="ts">
    import type {
        APIBillingPlan,
        APIBillingPlanPrice,
        APIBillingSubscription,
    } from "@palform/palform-typescript-openapi";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import PlanComparison from "../comparison/PlanComparison.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import SwitchPlanPreview from "./SwitchPlanPreview.svelte";

    interface Props {
        subscription: APIBillingSubscription;
    }

    let { subscription }: Props = $props();
    const orgCtx = getOrgContext();

    let loading = $state(false);
    let previewModalWithPriceId: APIBillingPlanPrice | undefined =
        $state(undefined);
    let previewModalAnnual = $state(false);

    let showPreviewModal = $state(false);
    let onSubscriptionSelect = $derived(
        async (e: {
            plan: APIBillingPlan;
            annual: boolean;
            trial: boolean;
        }) => {
            const plan = e.plan;
            const priceId = e.annual ? plan.price_annually : plan.price_monthly;

            previewModalWithPriceId = priceId;
            previewModalAnnual = e.annual;
            showPreviewModal = true;
        }
    );

    let onConfirmChange = $derived(async () => {
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
    });
</script>

<SectionHeading>Switch plan</SectionHeading>
<PlanComparison
    currentPriceId={subscription.stripe_plan_price_id}
    allowTrial={false}
    fixCurrency={subscription.currency}
    onselect={onSubscriptionSelect}
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
