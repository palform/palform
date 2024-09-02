import type { PricingContext } from "./contexts/pricing";

export function getCurrencySymbol(currency: string) {
    switch (currency) {
        case "gbp":
            return `£`;
        case "eur":
            return `€`;
        case "usd":
            return `$`;
        default:
            return "";
    }
}

export function formatCurrencyValue(value: number) {
    const v = value / 100;
    if (Number.isInteger(v)) {
        return v.toFixed(0);
    } else {
        return v.toFixed(1);
    }
}

export function formatCurrency(
    currency: PricingContext["currency"],
    value: number
) {
    return getCurrencySymbol(currency) + formatCurrencyValue(value);
}
