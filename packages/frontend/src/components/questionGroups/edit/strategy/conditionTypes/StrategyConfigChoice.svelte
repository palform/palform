<script lang="ts">
    import type { ConfigChoiceChoice } from "@palform/palform-typescript-openapi";
    import { Button, Checkbox, Label, Radio, Toggle } from "flowbite-svelte";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {
        configuration: ConfigChoiceChoice;
    }

    let { configuration, onsave }: Props = $props();
    let options: string[] = $state([]);
    let containsAny = $state(false);

    let onSave = $derived(() => {
        onsave({
            Choice: {
                options,
                contains_any: containsAny,
            },
        });
    });

    let onCheckboxChange = $derived((e: Event) => {
        const t = e.target as HTMLInputElement;

        if (t.checked) {
            options = [...options, t.value];
        } else {
            options = options.filter((e) => e !== t.value);
        }
    });
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
                    onchange={onCheckboxChange}
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

<Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
