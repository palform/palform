<script lang="ts">
    import type {
        APIQuestionConfigurationOneOf2Choice,
        APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Checkbox, Label, Radio, Toggle } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";

    export let configuration: APIQuestionConfigurationOneOf2Choice;
    let options: string[] = [];
    let containsAny = false;

    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    $: onSave = () => {
        dispatch("save", {
            Choice: {
                options,
                contains_any: containsAny,
            },
        });
    };

    $: onCheckboxChange = (e: Event) => {
        const t = e.target as HTMLInputElement;

        if (t.checked) {
            options = [...options, t.value];
        } else {
            options = options.filter((e) => e !== t.value);
        }
    };
</script>

<Label>
    {containsAny ? "Contains any of" : "Matches"}
    {configuration.multi ? "these" : "this"} option{configuration.multi
        ? "(s)"
        : ""}

    <div class="space-y-1 mt-2">
        {#each configuration.options as option (option)}
            {#if configuration.multi}
                <Checkbox
                    checked={options.includes(option)}
                    on:change={onCheckboxChange}
                    value={option}
                >
                    {option}
                </Checkbox>
            {:else}
                <Radio bind:group={options[0]} value={option}>
                    {option}
                </Radio>
            {/if}
        {/each}
    </div>
</Label>

{#if configuration.multi}
    <Toggle class="mt-4" bind:checked={containsAny}>
        Contains (instead of matches exactly)
    </Toggle>
{/if}

<Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
