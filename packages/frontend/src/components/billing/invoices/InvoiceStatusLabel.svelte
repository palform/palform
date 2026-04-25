<script lang="ts">
    import type { APIBillingInvoiceStatus } from "@palform/palform-typescript-openapi";
    import { Badge, type BadgeProps } from "flowbite-svelte";

    interface Props {
        status: APIBillingInvoiceStatus;
        class?: string;
    }

    let { status, class: className }: Props = $props();

    let color: BadgeProps["color"] = $derived.by(() => {
        switch (status) {
            case "Draft":
                return "blue";
            case "Open":
                return "yellow";
            case "Paid":
                return "green";
            case "Uncollectible":
                return "gray";
            case "Void":
                return "gray";
            default:
                return "primary";
        }
    });
</script>

<Badge class={className} {color}>
    {status}
</Badge>
