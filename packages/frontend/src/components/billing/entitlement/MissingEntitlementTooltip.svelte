<script lang="ts">
    import type { APIEntitlementInfo } from "@paltiverse/palform-typescript-openapi";
    import { isEntitled } from "../../../data/billing/entitlement";
    import { Tooltip } from "flowbite-svelte";
    import type { TooltipProps } from "flowbite-svelte/Tooltip.svelte";

    export let key: keyof APIEntitlementInfo;
    export let multi = false;
    export let placement: TooltipProps["placement"] = "right";
    $: entitled = isEntitled(key, multi);
</script>

{#if !$entitled}
    <Tooltip {placement}>Please upgrade to access this feature</Tooltip>
{/if}
