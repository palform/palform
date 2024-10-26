<script lang="ts">
    import { Label, Select, Spinner, Toggle } from "flowbite-svelte";
    import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
    import { billingAPI } from "../../api/main";
    import {
        freePlan,
        PricingFAQ,
        PricingPlan,
    } from "@paltiverse/palform-frontend-common";
    import { onMount } from "svelte";

    let annualPricing = true;
    let currency = "gbp";

    let plans: APIBillingPlan[] = [];
    let loading = true;

    $: reload = () => {
        loading = true;

        billingAPI
            .billingPlanList(currency)
            .then((resp) => {
                plans = resp.data.toSorted((a, b) => {
                    return a.price_monthly.amount - b.price_monthly.amount;
                });
            })
            .catch((e) => {
                plans = [];
                console.warn(e);
            })
            .finally(() => (loading = false));
    };

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

    <Label class="block w-80">
        Currency
        <Select
            class="mt-1"
            items={[
                { name: "£/GBP", value: "gbp" },
                { name: "€/EUR", value: "eur" },
                { name: "$/USD", value: "usd" },
            ]}
            bind:value={currency}
            on:change={reload}
        />
    </Label>
</div>

{#if loading}
    <div class="flex justify-center mt-10">
        <Spinner size="16" />
    </div>
{:else}
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
                on:click={onTrialClick}
            />
        {/each}
    </div>

    <PricingFAQ class="mt-8" />
{/if}
