<script lang="ts">
    import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
    import { getPricingContext, resolvePrice } from "../../contexts/pricing";
    import {
        formatCurrency,
        formatCurrencyValue,
        getCurrencySymbol,
    } from "../../util";
    import { Button } from "flowbite-svelte";
    import PricingFeatureItem from "./PricingFeatureItem.svelte";

    export let plan: APIBillingPlan;
    const ctx = getPricingContext();

    $: priceToShow = resolvePrice($ctx, plan);
    const onButtonClick = () => {
        window.location.href = "https://dash.palform.app/auth/signup";
    };
</script>

<div
    class={`p-6 dark:bg-slate-800 ${plan.highlight ? "border-2 border-primary-600 dark:border-primary-400 shadow shadow-primary-300" : "border-2 dark:border-slate-700"} rounded-xl`}
>
    <p
        class="font-display text-gray-600 dark:text-gray-300 text-lg mb-2 tracking-tight"
    >
        {plan.name}
    </p>

    <div class="flex items-end font-display">
        <p class="text-gray-600 dark:text-gray-400 text-xl me-1 mb-1">
            {getCurrencySymbol($ctx.currency)}
        </p>
        <p class="text-gray-950 text-5xl dark:text-gray-200">
            {formatCurrencyValue(priceToShow)}
        </p>
        <p class="ms-1 text-gray-600 mb-1 dark:text-gray-400">/month</p>
    </div>

    {#if $ctx.frequency === "annual"}
        <p class="text-gray-500 dark:text-gray-400 text-sm">
            Billed as {formatCurrency(
                $ctx.currency,
                plan.price_annually.amount
            )}/year
        </p>
    {/if}

    <Button
        class="mt-4 block w-full plausible-event-name=Plan+Select"
        size="lg"
        on:click={onButtonClick}
    >
        Try free for 14 days
    </Button>

    <hr class="border-t-slate-200 dark:border-t-slate-600 my-4" />

    <ul class="space-y-2">
        {#each plan.features as feature}
            <PricingFeatureItem>{feature}</PricingFeatureItem>
        {/each}
    </ul>
</div>
