<script lang="ts">
    import { Label, Select, Spinner, Toggle } from "flowbite-svelte";
    import type { APIBillingPlan } from "@palform/palform-typescript-openapi";
    import { billingAPI } from "../../api/main";
    import {
        freePlan,
        PricingFAQ,
        PricingPlan,
    } from "@palform/palform-frontend-common";
    import { onMount } from "svelte";

    let annualPricing = $state(true);
    let currency: string | undefined = $state(undefined);

    let plans: APIBillingPlan[] = $state([]);
    let loading = $state(true);

    let reload = $derived(() => {
        loading = true;

        billingAPI
            .billingPlanList(currency)
            .then((resp) => {
                plans = resp.data.data.toSorted((a, b) => {
                    return a.price_monthly.amount - b.price_monthly.amount;
                });
                currency = resp.data.currency;
            })
            .catch((e) => {
                plans = [];
                console.warn(e);
            })
            .finally(() => (loading = false));
    });

    onMount(() => {
        reload();
    });

    const onTrialClick = () => {
        window.location.href = "https://dash.palform.app/auth/signup";
    };
</script>

<div class="flex mb-6 gap-8 items-center">
    <Toggle bind:checked={annualPricing}>
        <span>
            Annual pricing (<strong>2 months FREE</strong>)
        </span>
    </Toggle>

    {#if currency !== undefined}
        <Label class="block w-80">
            Currency
            <Select
                class="mt-1"
                items={[
                    { name: "£/GBP", value: "gbp" },
                    { name: "€/EUR", value: "eur" },
                    { name: "$/USD", value: "usd" },
                    { name: "Fr/CHF", value: "chf" },
                ]}
                bind:value={currency}
                onchange={reload}
            />
        </Label>
    {/if}
</div>

{#if loading}
    <div class="flex justify-center mt-10">
        <Spinner size="16" />
    </div>
{:else if currency !== undefined}
    <div class="grid lg:grid-cols-3 gap-4">
        <PricingPlan
            plan={freePlan(currency)}
            currentPriceId={undefined}
            isFree
            annualBilling={annualPricing}
        />

        {#each plans as plan (plan.stripe_product_id)}
            <PricingPlan
                {plan}
                annualBilling={annualPricing}
                currentPriceId={undefined}
                allowTrial
                trialOnly
                showButton
                onclick={onTrialClick}
            />
        {/each}
    </div>

    <PricingFAQ class="mt-8" />
{/if}
