<script lang="ts">
    import type { APIQuestionConfigurationOneOf4 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import {
        Helper,
        Label,
        NumberInput,
        Toggle,
    } from "flowbite-svelte";
    import AddressSearch from "../../addressSearch/AddressSearch.svelte";
    import type { APIGenericLocation } from "@paltiverse/palform-client-js-extra-types/APIGenericLocation";
    import TextButton from "../../TextButton.svelte";
    import type { QuestionEditEvents } from "../../../data/contexts/formEditor";

    export let config: APIQuestionConfigurationOneOf4;
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    let focus = !!config.address.search_centre;

    $: onUpdate = () => {
        dispatch("update", config);
    };
    $: onAddressClick = (e: CustomEvent<{ location: APIGenericLocation }>) => {
        config.address.search_centre = e.detail.location;
        dispatch("update", config);
    };
    $: onEnterManuallyClick = () => {
        config.address.search_centre = { lat: 0, lng: 0 };
        dispatch("update", config);
    };
    $: onClearClick = () => {
        config.address.search_centre = null;
        dispatch("update", config);
    };
    $: onToggleChange = () => {
        if (focus === false) {
            config.address.search_centre = null;
            dispatch("update", config);
        }
    };
</script>

<Toggle bind:checked={focus} on:change={onToggleChange}>
    Focus the search on a specific location
</Toggle>

{#if focus}
    {#if !config.address.search_centre}
        <AddressSearch class="mt-4" on:select={onAddressClick} />
        <Helper class="mt-1">
            <TextButton on:click={onEnterManuallyClick}>
                Enter coordinates manually
            </TextButton>
        </Helper>
    {:else}
        <div class="flex items-center gap-8 mt-4">
            <Label>
                Latitude
                <NumberInput
                    class="mt-1"
                    bind:value={config.address.search_centre.lat}
                    on:input={onUpdate}
                />
            </Label>
            <Label>
                Longitude
                <NumberInput
                    class="mt-1"
                    bind:value={config.address.search_centre.lng}
                    on:input={onUpdate}
                />
            </Label>
        </div>

        <TextButton class="mt-1" on:click={onClearClick}>
            Search for address
        </TextButton>
    {/if}
{/if}
