<script lang="ts">
    import type { ConfigChoiceMatrixChoiceMatrix } from "@palform/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {
        configuration: ConfigChoiceMatrixChoiceMatrix;
    }

    let { configuration, onsave }: Props = $props();

    let row = $state("");
    let column = $state("");
    let onSave = $derived(() => {
        onsave({
            ChoiceMatrix: {
                row,
                column,
            },
        });
    });
</script>

<Label>
    Selected value for row
    <Select
        class="mt-2"
        items={configuration.rows.map((e) => ({ name: e, value: e }))}
        bind:value={row}
    />
</Label>

<Label class="mt-4">
    {configuration.multi_cols ? "contains" : "is"}
    <Select
        class="mt-2"
        items={configuration.columns.map((e) => ({ name: e, value: e }))}
        bind:value={column}
    />
</Label>

{#if row !== "" && column !== ""}
    <Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
{/if}
