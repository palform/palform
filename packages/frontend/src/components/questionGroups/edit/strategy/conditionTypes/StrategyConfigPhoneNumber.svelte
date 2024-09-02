<script lang="ts">
    import { type APIQuestionGroupStepStrategyJumpCaseConditionMatcher } from "@paltiverse/palform-typescript-openapi";
    import { Button, Label } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import CallingCodeDropdown from "../../../../callingCode/CallingCodeDropdown.svelte";
    import { showFailureToast } from "../../../../../data/toast";

    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    let callingCode = "";
    $: onSave = async () => {
        if (callingCode === "") {
            await showFailureToast("Please select a calling code");
            return;
        }
        dispatch("save", {
            PhoneNumber: {
                calling_code: callingCode,
            },
        });
    };
</script>

<Label>
    Selected calling code is
    <CallingCodeDropdown bind:value={callingCode} class="block mt-2" />
</Label>

<Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
