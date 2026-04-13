<script lang="ts">
    import type { AuditLogTargetResourceEnum } from "@palform/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import PalformDatePicker from "../../datePicker/PalformDatePicker.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faRotateRight } from "@fortawesome/free-solid-svg-icons";
    import type { DateTime } from "luxon";
    import type { AuditLogRequestFilters } from "../../../data/audit/filters";
    import { onMount } from "svelte";

    interface Props {
        filters: AuditLogRequestFilters;
        disabled?: boolean;
        class?: string;
        onreload: () => void;
    }

    let {
        filters = $bindable(),
        disabled = false,
        class: className,
        onreload,
    }: Props = $props();

    const resourceItems: {
        name: string;
        value: AuditLogTargetResourceEnum | null;
    }[] = [
        { name: "All", value: null },
        { name: "Forms", value: "Form" },
        { name: "Branding schemes", value: "Branding" },
        { name: "Teams", value: "Team" },
        { name: "Team members", value: "TeamMember" },
        { name: "Organisation metadata", value: "Organisation" },
        { name: "Organisation members", value: "OrganisationMember" },
        {
            name: "Authentication configuration",
            value: "OrganisationAuthConfig",
        },
        { name: "Subdomain", value: "OrganisationSubdomain" },
    ];

    let startDate = $state<DateTime | null>(null);
    let endDate = $state<DateTime | null>(null);

    onMount(() => {
        startDate = filters.from ? parseServerTime(filters.from) : null;
        endDate = filters.to ? parseServerTime(filters.to) : null;
    });

    const onStartChange = (e: DateTime | null) => {
        filters.from = e ? e.toISO() : null;
    };
    const onEndChange = (e: DateTime | null) => {
        filters.to = e ? e.toISO() : null;
    };
</script>

<div class={`flex gap-x-4 ${className ?? ""}`}>
    <Label>
        Resource
        <Select
            bind:value={filters.resource}
            items={resourceItems}
            {disabled}
        />
    </Label>
    <Label>
        Start
        <PalformDatePicker
            bind:value={startDate}
            max={endDate}
            onchange={onStartChange}
            {disabled}
        />
    </Label>
    <Label>
        End
        <PalformDatePicker
            bind:value={endDate}
            min={startDate}
            onchange={onEndChange}
            {disabled}
        />
    </Label>

    <Button class="mt-4 ml-4" outline onclick={() => onreload()} {disabled}>
        <FontAwesomeIcon icon={faRotateRight} />
    </Button>
</div>
