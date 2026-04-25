<script lang="ts">
    import type { ConfigHiddenHidden } from "@palform/palform-typescript-openapi";
    import { Button, Input, Label } from "flowbite-svelte";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {
        configuration: ConfigHiddenHidden;
    }

    let { configuration, onsave }: Props = $props();

    let value = $state("");
    let onSave = $derived(() => {
        onsave({
            Hidden: {
                value,
            },
        });
    });
</script>

<Label>
    Value of <code>{configuration.parameter_name}</code> is
    <Input class="mt-2" bind:value />
</Label>

<Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
