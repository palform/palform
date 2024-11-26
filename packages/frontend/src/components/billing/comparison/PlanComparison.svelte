<script lang="ts">
    import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import CardGrid from "../../CardGrid.svelte";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import { Label, Select, Toggle } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import { createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";
    import {
        freePlan,
        PricingFAQ,
        PricingPlan,
    } from "@paltiverse/palform-frontend-common";

    export let disabled = false;
    export let currentPriceId: string | undefined = undefined;
    export let allowTrial = true;
    export let fixCurrency: string | undefined = undefined;

    let plans: APIBillingPlan[] | undefined = [];
    let plansLoading = true;
    let currency: string | undefined = fixCurrency;

    $: (() => {
        plansLoading = true;
        APIs.billingPlans()
            .then((a) => a.billingPlanList(currency))
            .then((resp) => {
                plans = resp.data.data.toSorted(
                    (a, b) => a.price_monthly.amount - b.price_monthly.amount
                );
                currency = resp.data.currency;
                plansLoading = false;
            });
    })();

    let useAnnual = true;
    const dispatch = createEventDispatcher<{
        select: { plan: APIBillingPlan; annual: boolean; trial: boolean };
    }>();
    $: onSelect = (plan: APIBillingPlan, trial: boolean) => {
        dispatch("select", {
            plan,
            annual: useAnnual,
            trial,
        });
    };
</script>

{#if plansLoading}
    <SkeletonPrimitive height="30px" className="mb-4" />
    <SkeletonPrimitive height="30px" className="mb-4" />
    <CardGrid>
        <SkeletonPrimitive height="520px" />
        <SkeletonPrimitive height="520px" />
        <SkeletonPrimitive height="520px" />
    </CardGrid>
    <SkeletonPrimitive height="60px" className="mt-8" />
    <SkeletonPrimitive height="60px" className="mt-2" />
    <SkeletonPrimitive height="60px" className="mt-2" />
{:else if plans !== undefined}
    <div in:fade>
        <Toggle class="mb-4" bind:checked={useAnnual} {disabled}>
            Annual billing (2 months free!)
        </Toggle>

        {#if fixCurrency === undefined && currency !== undefined}
            <Label class="mb-4 inline-block">
                Currency
                <Select
                    class="mt-2"
                    items={[
                        { name: "£ (GBP)", value: "gbp" },
                        { name: "€ (EUR)", value: "eur" },
                        { name: "$ (USD)", value: "usd" },
                    ]}
                    bind:value={currency}
                />
            </Label>
        {/if}

        <InfoText class="mb-8">
            Prices shown <strong>exclude</strong> VAT, which may be applied at checkout.
        </InfoText>

        <CardGrid>
            {#if currentPriceId === undefined && currency !== undefined}
                <PricingPlan
                    isFree
                    plan={freePlan(currency)}
                    showButton={true}
                    annualBilling={useAnnual}
                    {currentPriceId}
                />
            {/if}

            {#each plans as plan, index (plan.stripe_product_id)}
                <PricingPlan
                    {plan}
                    everythingIn={index > 0 ? plans[index - 1].name : undefined}
                    showButton
                    on:click={(e) => onSelect(plan, e.detail)}
                    annualBilling={useAnnual}
                    {disabled}
                    {allowTrial}
                    {currentPriceId}
                />
            {/each}
        </CardGrid>

        <PricingFAQ class="mt-8" />
    </div>
{/if}
