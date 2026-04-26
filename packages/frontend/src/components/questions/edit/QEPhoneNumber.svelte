<script lang="ts">
    import type {
        APICountryWithCallingCode,
        ConfigPhoneNumber,
    } from "@palform/palform-typescript-openapi";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";
    import { Alert, Button, Label, Li, List } from "flowbite-svelte";
    import CallingCodeDropdown from "../../callingCode/CallingCodeDropdown.svelte";
    import TextButton from "../../TextButton.svelte";

    interface Props {
        config: ConfigPhoneNumber;
    }

    let { config = $bindable() }: Props = $props();
    let ctx = getFormEditorCtx();

    let defaultCallingCode = $state(
        config.phone_number.default_calling_code ?? ""
    );

    let onDefaultCallingCodeUpdate = $derived(
        (newVal: APICountryWithCallingCode | null) => {
            config.phone_number.default_calling_code = newVal
                ? `+${newVal?.calling_code}`
                : null;
            defaultCallingCode = config.phone_number.default_calling_code ?? "";
        }
    );

    let addingAllowedCode = $state(false);
    let newCallingCode = $state("");
    let onAddAllowedCallingCode = $derived(() => {
        if (newCallingCode === "") return;
        config.phone_number.allowed_calling_codes = [
            ...(config.phone_number.allowed_calling_codes ?? []),
            newCallingCode,
        ];
        addingAllowedCode = false;
        newCallingCode = "";
    });
    let onDeleteAllowedCallingCode = $derived((index: number) => {
        const newConfig = [
            ...(config.phone_number.allowed_calling_codes ?? []),
        ];
        newConfig.splice(index, 1);
        config.phone_number.allowed_calling_codes = newConfig;
    });
</script>

<fieldset>
    <Label>
        Default calling code
        <div class="block mt-1">
            <CallingCodeDropdown
                disabled={$ctx.loading}
                value={defaultCallingCode}
                onupdate={onDefaultCallingCodeUpdate}
                allowedValues={config.phone_number.allowed_calling_codes}
            />
        </div>
    </Label>

    {#if defaultCallingCode !== ""}
        <TextButton
            class="text-xs mt-2"
            onclick={() => onDefaultCallingCodeUpdate(null)}
        >
            Clear
        </TextButton>
    {/if}
</fieldset>

{#if defaultCallingCode !== "" && config.phone_number.allowed_calling_codes !== undefined && config.phone_number.allowed_calling_codes.length > 0 && !config.phone_number.allowed_calling_codes.includes(defaultCallingCode)}
    <Alert color="orange" class="mt-4">
        The default calling code is not in the list of allowed calling codes.
        This may result in unexpected behaviour. Please choose a default calling
        code from the list.
    </Alert>
{/if}

<fieldset class="mt-4">
    <Label>Allowed calling codes</Label>

    {#if config.phone_number.allowed_calling_codes?.length !== 0}
        <List>
            {#each config.phone_number.allowed_calling_codes as allowed_code, index}
                <Li class="text-sm text-gray-700 dark:text-gray-200">
                    {allowed_code}
                    (<TextButton
                        class="inline"
                        disabled={$ctx.loading}
                        onclick={() => onDeleteAllowedCallingCode(index)}
                    >
                        delete
                    </TextButton>)
                </Li>
            {/each}
        </List>
    {:else}
        <Alert color="gray" class="mt-1">
            By default, all calling codes are allowed.
        </Alert>
    {/if}

    {#if !addingAllowedCode}
        <Button
            color="light"
            size="xs"
            class="mt-2"
            onclick={() => (addingAllowedCode = true)}
            disabled={$ctx.loading}
        >
            Add calling code
        </Button>
    {:else}
        <div>
            <CallingCodeDropdown
                disabled={$ctx.loading}
                bind:value={newCallingCode}
                autoOpen
                class="mt-2"
            />
            {#if newCallingCode !== ""}
                <Button
                    onclick={onAddAllowedCallingCode}
                    disabled={$ctx.loading}
                >
                    Add
                </Button>
            {/if}
        </div>
        <TextButton
            class="text-xs mt-1"
            onclick={() => (addingAllowedCode = false)}
        >
            Cancel
        </TextButton>
    {/if}
</fieldset>
