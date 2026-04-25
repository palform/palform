<script lang="ts">
    import { Button, Label } from "flowbite-svelte";
    import CallingCodeDropdown from "../../../../callingCode/CallingCodeDropdown.svelte";
    import { showFailureToast } from "../../../../../data/toast";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {}
    let { onsave }: Props = $props();

    let callingCode = $state("");
    let onSave = $derived(async () => {
        if (callingCode === "") {
            await showFailureToast("Please select a calling code");
            return;
        }
        onsave({
            PhoneNumber: {
                calling_code: callingCode,
            },
        });
    });
</script>

<Label>
    Selected calling code is
    <CallingCodeDropdown bind:value={callingCode} class="block mt-2" />
</Label>

<Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
