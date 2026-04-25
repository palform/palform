<script lang="ts">
    import type { APIBillingPlan } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import CardGrid from "../../CardGrid.svelte";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import { Label, Select, Toggle } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import { fade } from "svelte/transition";
    import {
        freePlan,
        PricingFAQ,
        PricingPlan,
    } from "@palform/palform-frontend-common";

    interface Props {
        disabled?: boolean;
        currentPriceId?: string | undefined;
        allowTrial?: boolean;
        fixCurrency?: string | undefined;
        onselect: (details: {
            plan: APIBillingPlan;
            annual: boolean;
            trial: boolean;
        }) => void;
    }

    let {
        disabled = false,
        currentPriceId = undefined,
        allowTrial = true,
        fixCurrency = undefined,
        onselect,
    }: Props = $props();

    let plans: APIBillingPlan[] | undefined = $state([]);
    let plansLoading = $state(true);
    let currency: string | undefined = $state(fixCurrency);

    $effect(() => {
        plansLoading = true;
        currency;

        APIs.billingPlans()
            .then((a) => a.billingPlanList(currency))
            .then((resp) => {
                plans = resp.data.data.toSorted(
                    (a, b) => a.price_monthly.amount - b.price_monthly.amount
                );
                currency = resp.data.currency;
                plansLoading = false;
            });
    });

    let useAnnual = $state(true);
    let onSelect = $derived((plan: APIBillingPlan, trial: boolean) => {
        onselect({
            plan,
            annual: useAnnual,
            trial,
        });
    });
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
                        { name: "Fr (CHF)", value: "chf" },
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
                    onclick={(e) => onSelect(plan, e)}
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
