<script lang="ts">
    import type {
        AuditLogListRequest,
        AuditLogTargetResourceEnum,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import PalformDatePicker from "../../datePicker/PalformDatePicker.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faRotateRight } from "@fortawesome/free-solid-svg-icons";
    import type { DateTime } from "luxon";
    import { createEventDispatcher } from "svelte";

    export let filters: AuditLogListRequest;
    export let disabled = false;
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

    $: startDate = filters.from ? parseServerTime(filters.from) : null;
    $: endDate = filters.to ? parseServerTime(filters.to) : null;
    const onStartChange = (e: CustomEvent<DateTime | null>) => {
        filters.from = e.detail ? e.detail.toISO() : null;
    };
    const onEndChange = (e: CustomEvent<DateTime | null>) => {
        filters.to = e.detail ? e.detail.toISO() : null;
    };

    const dispatch = createEventDispatcher<{ reload: undefined }>();
</script>

<div class={`flex gap-x-4 ${$$props.class}`}>
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
            on:change={onStartChange}
            {disabled}
        />
    </Label>
    <Label>
        End
        <PalformDatePicker
            bind:value={endDate}
            min={startDate}
            on:change={onEndChange}
            {disabled}
        />
    </Label>

    <Button
        class="mt-4 ml-4"
        outline
        on:click={() => dispatch("reload")}
        {disabled}
    >
        <FontAwesomeIcon icon={faRotateRight} />
    </Button>
</div>
