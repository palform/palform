<script lang="ts">
    import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
    import { Badge, Button, Card } from "flowbite-svelte";
    import {
        formatCurrency,
        formatDecimalCurrency,
        getCurrencySymbol,
        PricingFeatureItem,
    } from "@paltiverse/palform-frontend-common";
    import { createEventDispatcher } from "svelte";

    export let plan: APIBillingPlan;
    export let everythingIn: string | undefined = undefined;
    export let showButton = false;
    export let annualBilling: boolean;
    export let disabled = false;
    export let currentPriceId: string | undefined;
    export let allowTrial = true;
    export let trialOnly = false;

    const dispatch = createEventDispatcher<{ click: boolean }>();

    $: isCurrent = annualBilling
        ? plan.price_annually.stripe_price_id === currentPriceId
        : plan.price_monthly.stripe_price_id === currentPriceId;
</script>

<Card padding="xl" size="none" shadow={false} class="rounded-2xl">
    <h4 class="text-xl text-gray-600 dark:text-gray-300">{plan.name}</h4>
    <div class="flex items-baseline text-gray-900 dark:text-white mt-4">
        <p class="text-3xl font-semibold">
            {getCurrencySymbol(plan.currency)}
        </p>
        <h3 class="text-5xl tracking-tight font-extrabold">
            {formatDecimalCurrency(
                annualBilling
                    ? plan.price_annually.amount / 12
                    : plan.price_monthly.amount,
                true,
            )}
        </h3>
        <p class="ms-1 text-gray-500 dark:text-gray-400 text-xl">/month</p>
    </div>
    {#if annualBilling}
        <p class="mt-2">
            <Badge color="green">
                Save {formatCurrency(
                    plan.currency,
                    plan.price_monthly.amount * 12 - plan.price_annually.amount,
                    true,
                )}
            </Badge>
        </p>
        <p class="mt-1 text-sm">
            Charged as {formatCurrency(
                plan.currency,
                plan.price_annually.amount,
            )} annually
        </p>
    {/if}

    {#if showButton}
        {#if isCurrent}
            <Button class="mt-6" size="lg" disabled color="light">
                Current plan
            </Button>
        {:else if allowTrial}
            <Button
                class="mt-6"
                size="lg"
                on:click={() => dispatch("click", true)}
                {disabled}
            >
                Try free for 14 days
            </Button>
            {#if !trialOnly}
                <Button
                    class="mt-4"
                    outline
                    {disabled}
                    on:click={() => dispatch("click", false)}
                >
                    Buy now
                </Button>
            {/if}
        {:else}
            <Button
                class="mt-6"
                {disabled}
                size="lg"
                on:click={() => dispatch("click", false)}
            >
                Buy now
            </Button>
        {/if}
    {/if}

    <ul class="mt-7 space-y-4">
        {#if everythingIn}
            <PricingFeatureItem plus>
                Everything in {everythingIn}
            </PricingFeatureItem>
        {/if}
        {#each plan.features as feature}
            <PricingFeatureItem>
                {feature}
            </PricingFeatureItem>
        {/each}
    </ul>
</Card>
