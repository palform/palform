import type { APIBillingPlan } from "@paltiverse/palform-typescript-openapi";

export function formatDecimalCurrency(amount: number, round = false) {
    const v = amount / 100;
    if (Number.isInteger(v) || round) {
        return v.toFixed(0);
    } else {
        return v.toFixed(2);
    }
}

export function getCurrencySymbol(currency: string) {
    switch (currency.toLowerCase()) {
        case "gbp":
            return "£";
        case "eur":
            return "€";
        case "usd":
            return "US$";
        default:
            return "";
    }
}
export function formatCurrency(
    currency: string,
    amount: number,
    round = false
) {
    return `${getCurrencySymbol(currency)}${formatDecimalCurrency(amount, round)}`;
}

export function freePlan(currency: string): APIBillingPlan {
    return {
        name: "Free",
        stripe_product_id: "",
        price_monthly: { stripe_price_id: "", amount: 0 },
        price_annually: { stripe_price_id: "", amount: 0 },
        currency: currency,
        highlight: false,
        features: [
            "10 forms",
            "1 user",
            "Unlimited submissions",
            "20 questions",
            "All question types",
            "Unlimited file uploads",
            "Conditional branching",
            "End-to-end encryption",
        ],
    };
}
