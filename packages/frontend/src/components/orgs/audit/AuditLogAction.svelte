<script lang="ts">
    import type { AuditLogVerbEnum } from "@paltiverse/palform-typescript-openapi";
    import { Badge } from "flowbite-svelte";
    import type { BadgeProps } from "flowbite-svelte/Badge.svelte";

    export let action: AuditLogVerbEnum;
    let color: BadgeProps["color"] =
        action === "Create"
            ? "green"
            : action === "Update"
              ? "yellow"
              : action === "Delete"
                ? "red"
                : "blue";
</script>

<Badge {color}>
    {action}
</Badge>
