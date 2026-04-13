<script lang="ts">
    import { Button, Helper, Input, Label, Toggle } from "flowbite-svelte";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    let value = $state("");
    let caseSensitive = $state(true);
    let contains = $state(false);

    interface Props extends StrategyMatcherEventProps {}
    let { onsave }: Props = $props();

    let onSave = $derived(() => {
        onsave({
            Text: {
                value,
                contains,
                case_sensitive: caseSensitive,
            },
        });
    });
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

<Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
