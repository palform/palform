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
    round = false,
) {
    return `${getCurrencySymbol(currency)}${formatDecimalCurrency(amount, round)}`;
}
