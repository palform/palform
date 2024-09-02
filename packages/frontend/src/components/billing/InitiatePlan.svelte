<script lang="ts">
    import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
    import PlanComparison from "./comparison/PlanComparison.svelte";
    import { APIs } from "../../data/common";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { stripeRedirectURL } from "../../data/billing/util";

    const orgCtx = getOrgContext();
    export let callbackPath = "/settings/billing";
    let initiateLoading = false;
    $: onPlanSelect = async (
        plan: APIBillingPlan,
        annual: boolean,
        trial: boolean
    ) => {
        initiateLoading = true;
        const resp = await APIs.billingPlans().then((a) =>
            a.billingPlanInitiate($orgCtx.org.id, {
                stripe_plan_price_id: annual
                    ? plan.price_annually.stripe_price_id
                    : plan.price_monthly.stripe_price_id,
                success_url: stripeRedirectURL(
                    $orgCtx.org.id,
                    callbackPath
                ).toString(),
                trial,
            })
        );

        window.location.href = resp.data;
        initiateLoading = false;
    };
</script>

<PlanComparison
    disabled={initiateLoading}
    on:select={({ detail: { plan, annual, trial } }) =>
        onPlanSelect(plan, annual, trial)}
/>
