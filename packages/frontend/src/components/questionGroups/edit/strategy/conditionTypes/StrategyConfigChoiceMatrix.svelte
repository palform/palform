<script lang="ts">
    import type {
        APIQuestionConfigurationOneOf8ChoiceMatrix,
        APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";

    export let configuration: APIQuestionConfigurationOneOf8ChoiceMatrix;
    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    let row = "";
    let column = "";
    $: onSave = () => {
        dispatch("save", {
            ChoiceMatrix: {
                row,
                column,
            },
        });
    };
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
    <Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
{/if}
