import type { DirectionOperator } from "@paltiverse/palform-typescript-openapi";

export const comparisonItems: {
    value: DirectionOperator;
    name: string;
}[] = [
    { value: "Equal", name: "=" },
    { value: "LessThan", name: "<" },
    { value: "LessThanEqualTo", name: "<=" },
    { value: "GreaterThan", name: ">" },
    { value: "GreaterThanEqualTo", name: ">=" },
];

export const comparisonSymbol = (value: DirectionOperator) => {
    return comparisonItems.find((e) => e.value === value)?.name;
};
