<script lang="ts">
    import type {
        APIQuestionConfigurationOneOf10Hidden,
        APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Input, Label } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";

    export let configuration: APIQuestionConfigurationOneOf10Hidden;
    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    let value = "";
    $: onSave = () => {
        dispatch("save", {
            Hidden: {
                value,
            },
        });
    };
</script>

<Label>
    Value of <code>{configuration.parameter_name}</code> is
    <Input class="mt-2" bind:value />
</Label>

<Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
