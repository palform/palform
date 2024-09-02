<script lang="ts">
    import { writable } from "svelte/store";
    import {
        setPricingContext,
        type PricingContext,
    } from "../../contexts/pricing";
    import PricingPlan from "./PricingPlan.svelte";
    import { Label, Select, Spinner, Toggle } from "flowbite-svelte";
    import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
    import { billingAPI } from "../../api/main";
    import { onMount } from "svelte";
    import PricingFaq from "./PricingFAQ.svelte";

    const ctx = writable<PricingContext>({
        currency: "gbp",
        frequency: "annual",
    });
    setPricingContext(ctx);

    let annualPricing = true;
    $: onAnnualChanged = () => {
        $ctx.frequency = annualPricing ? "annual" : "monthly";
    };

    let plans: APIBillingPlan[] = [];
    let loading = true;

    onMount(() => {
        let previousCurrency = "";
        return ctx.subscribe((val) => {
            if (val.currency === previousCurrency) return;
            previousCurrency = val.currency;

            loading = true;

            billingAPI
                .billingPlanList(val.currency)
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
        });
    });
</script>

<div class="flex mb-6 gap-8 items-center">
    <Toggle bind:checked={annualPricing} on:change={onAnnualChanged}>
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
            bind:value={$ctx.currency}
        />
    </Label>
</div>

{#if loading}
    <div class="flex justify-center mt-10">
        <Spinner size="16" />
    </div>
{:else}
    <div class="grid lg:grid-cols-3 gap-4">
        {#each plans as plan (plan.stripe_product_id)}
            <PricingPlan {plan} />
        {/each}
    </div>

    <PricingFaq class="mt-8" />
{/if}
