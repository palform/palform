export default function expiryTimeOptions(
    never: false,
): { name: string; value: number }[];
export default function expiryTimeOptions(
    never: true,
): { name: string; value: number | null }[];
export default function expiryTimeOptions(never: boolean) {
    const items: { name: string; value: number | null }[] = [
        { name: "1 day", value: 24 * 60 },
        { name: "2 days", value: 2 * 24 * 60 },
        { name: "1 week", value: 7 * 24 * 60 },
        { name: "2 weeks", value: 2 * 7 * 24 * 60 },
        { name: "1 month", value: 30 * 24 * 60 },
        { name: "2 months", value: 2 * 30 * 24 * 60 },
        { name: "3 months", value: 3 * 30 * 24 * 60 },
        { name: "6 months", value: 6 * 30 * 24 * 60 },
        { name: "1 year", value: 365 * 24 * 60 },
    ];

    if (never) {
        items.unshift({
            name: "Never",
            value: null,
        });
    }

    return items;
}
