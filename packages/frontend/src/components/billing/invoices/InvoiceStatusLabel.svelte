<script lang="ts">
    import type { APIBillingInvoiceStatus } from "@paltiverse/palform-typescript-openapi";
    import { Badge } from "flowbite-svelte";
    import type { BadgeProps } from "flowbite-svelte/Badge.svelte";

    export let status: APIBillingInvoiceStatus;

    let color: BadgeProps["color"] = "primary";
    $: {
        switch (status) {
            case "Draft":
                color = "blue";
            case "Open":
                color = "yellow";
            case "Paid":
                color = "green";
            case "Uncollectible":
                color = "dark";
            case "Void":
                color = "dark";
        }
    }
</script>

<Badge class={$$props.class} color={"green"}>
    {status}
</Badge>
