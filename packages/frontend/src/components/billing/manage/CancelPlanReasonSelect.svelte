<script lang="ts">
    import type { CancelPlanRequestReason } from "@palform/palform-typescript-openapi";
    import { Select } from "flowbite-svelte";

    interface Props {
        value: CancelPlanRequestReason;
        class?: string;
    }
    let { value = $bindable(), class: className }: Props = $props();

    const reasonItems: { name: string; value: CancelPlanRequestReason }[] = [
        { name: "Poor customer service", value: "CustomerService" },
        { name: "The quality of Palform is too low", value: "LowQuality" },
        { name: "I am missing some features", value: "MissingFeatures" },
        { name: "I am switching to another service", value: "SwitchedService" },
        {
            name: "Palform is too complicated for my needs",
            value: "TooComplex",
        },
        { name: "Palform costs too much", value: "TooExpensive" },
        { name: "I don't use Palform enough", value: "Unused" },
        { name: "Other/I don't want to specify", value: "Other" },
    ];
</script>

<Select class={className} items={reasonItems} bind:value />
