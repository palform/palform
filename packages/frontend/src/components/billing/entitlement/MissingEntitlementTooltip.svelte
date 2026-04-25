<script lang="ts">
    import type { APIEntitlementInfo } from "@palform/palform-typescript-openapi";
    import { isEntitled } from "../../../data/billing/entitlement";
    import { Tooltip, type TooltipProps } from "flowbite-svelte";

    interface Props {
        key: keyof APIEntitlementInfo;
        multi?: boolean;
        placement?: TooltipProps["placement"];
    }

    let { key, multi = false, placement = "right" }: Props = $props();
    let entitled = $derived(isEntitled(key, multi));
</script>

{#if !$entitled}
    <Tooltip {placement}>Please upgrade to access this feature</Tooltip>
{/if}
