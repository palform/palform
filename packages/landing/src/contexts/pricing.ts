import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";
import { getContext, setContext } from "svelte";
import type { Writable } from "svelte/store";

export interface PricingContext {
    frequency: "annual" | "monthly";
    currency: "gbp" | "eur" | "usd";
}

export function setPricingContext(ctx: Writable<PricingContext>) {
    setContext("pricing_settings", ctx);
}
export function getPricingContext() {
    return getContext<Writable<PricingContext>>("pricing_settings");
}

export interface PlanPriceCurrencies {
    gbp: number;
    eur: number;
}

export interface PlanPriceConfig {
    annual: PlanPriceCurrencies;
    monthly: PlanPriceCurrencies;
}

export function resolvePrice(ctx: PricingContext, config: APIBillingPlan) {
    if (ctx.frequency === "annual") {
        return config.price_annually.amount / 12;
    } else if (ctx.frequency === "monthly") {
        return config.price_monthly.amount;
    }

    throw new Error(`Unrecognised frequency ${ctx.frequency}`);
}
