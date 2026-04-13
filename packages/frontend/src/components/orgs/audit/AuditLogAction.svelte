<script lang="ts">
    import type { AuditLogVerbEnum } from "@palform/palform-typescript-openapi";
    import { Badge, type BadgeProps } from "flowbite-svelte";

    interface Props {
        action: AuditLogVerbEnum;
    }

    let { action }: Props = $props();
    let color: BadgeProps["color"] = $derived(
        action === "Create"
            ? "green"
            : action === "Update"
              ? "yellow"
              : action === "Delete"
                ? "red"
                : "blue"
    );
</script>

<Badge {color}>
    {action}
</Badge>
