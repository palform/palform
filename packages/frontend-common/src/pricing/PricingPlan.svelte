<script lang="ts">
    import type { APIBillingPlan } from "@palform/palform-typescript-openapi";
    import { Badge, Button, Card } from "flowbite-svelte";
    import {
        formatCurrency,
        formatDecimalCurrency,
        getCurrencySymbol,
    } from "../data/util/pricing";
    import PricingFeatureItem from "./PricingFeatureItem.svelte";

    interface Props {
        plan: APIBillingPlan;
        everythingIn?: string | undefined;
        showButton?: boolean;
        annualBilling: boolean;
        disabled?: boolean;
        currentPriceId: string | undefined;
        allowTrial?: boolean;
        trialOnly?: boolean;
        isFree?: boolean;
        onclick?: (trial: boolean) => void;
    }

    let {
        plan,
        everythingIn = undefined,
        showButton = false,
        annualBilling,
        disabled = false,
        currentPriceId,
        allowTrial = true,
        trialOnly = false,
        isFree = false,
        onclick,
    }: Props = $props();

    let isCurrent = $derived(
        annualBilling
            ? plan.price_annually.stripe_price_id === currentPriceId
            : plan.price_monthly.stripe_price_id === currentPriceId
    );
</script>

<Card shadow={"xs"} class="rounded-2xl p-8 w-full max-w-full">
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
                false
            )}
        </h3>
        <p class="ms-1 text-gray-500 dark:text-gray-400 text-xl">/month</p>
    </div>
    {#if annualBilling && !isFree}
        <p class="mt-2">
            <Badge color="green">
                Save {formatCurrency(
                    plan.currency,
                    plan.price_monthly.amount * 12 - plan.price_annually.amount,
                    true
                )}
            </Badge>
        </p>
        <p class="mt-1 text-sm">
            Charged as {formatCurrency(
                plan.currency,
                plan.price_annually.amount
            )} annually
        </p>
    {/if}

    {#if showButton}
        {#if isCurrent || isFree}
            <Button
                class={`${isCurrent ? "mt-6" : "mt-20 mb-14"}`}
                size="lg"
                disabled
                color="light"
            >
                Current plan
            </Button>
        {:else if allowTrial}
            <Button
                class="mt-6"
                size="lg"
                onclick={() => onclick?.(true)}
                {disabled}
            >
                Try free for 14 days
            </Button>
            {#if !trialOnly}
                <Button
                    class="mt-4"
                    outline
                    {disabled}
                    onclick={() => onclick?.(false)}
                >
                    Buy now
                </Button>
            {/if}
        {:else}
            <Button
                class="mt-6"
                {disabled}
                size="lg"
                onclick={() => onclick?.(false)}
            >
                Buy now
            </Button>
        {/if}
    {/if}

    <ul class="mt-8 space-y-4">
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
