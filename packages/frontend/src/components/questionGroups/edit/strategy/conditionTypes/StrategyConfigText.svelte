<script lang="ts">
    import { Button, Helper, Input, Label, Toggle } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import type { APIQuestionGroupStepStrategyJumpCaseConditionMatcher } from "@paltiverse/palform-typescript-openapi";

    let value = "";
    let caseSensitive = true;
    let contains = false;

    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    $: onSave = () => {
        dispatch("save", {
            Text: {
                value,
                contains,
                case_sensitive: caseSensitive,
            },
        });
    };
</script>

<Label>
    Value
    <Input class="mt-2" bind:value />
    <Helper class="mt-2">
        The value to be entered to match this condition
    </Helper>
</Label>

<Toggle class="mt-4" bind:checked={caseSensitive}>Case sensitive</Toggle>

<Toggle class="mt-4" bind:checked={contains}>
    Contains value (instead of exactly matches)
</Toggle>

<Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
