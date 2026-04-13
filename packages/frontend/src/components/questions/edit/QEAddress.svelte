<script lang="ts">
    import type { ConfigAddress } from "@palform/palform-typescript-openapi";
    import { Helper, Input, Label, Toggle } from "flowbite-svelte";
    import AddressSearch from "../../addressSearch/AddressSearch.svelte";
    import type { APIGenericLocation } from "@palform/palform-client-js-extra-types/APIGenericLocation";
    import TextButton from "../../TextButton.svelte";

    interface Props {
        config: ConfigAddress;
    }

    let { config = $bindable() }: Props = $props();

    let focus = $state(!!config.address.search_centre);

    let onAddressClick = $derived(
        (e: CustomEvent<{ location: APIGenericLocation }>) => {
            config.address.search_centre = e.detail.location;
        }
    );
    let onEnterManuallyClick = $derived(() => {
        config.address.search_centre = { lat: 0, lng: 0 };
    });
    let onClearClick = $derived(() => {
        config.address.search_centre = null;
    });
    let onToggleChange = $derived(() => {
        if (focus === false) {
            config.address.search_centre = null;
        }
    });
</script>

<Toggle bind:checked={focus} onchange={onToggleChange}>
    Focus the search on a specific location
</Toggle>

{#if focus}
    {#if !config.address.search_centre}
        <AddressSearch class="mt-4" on:select={onAddressClick} />
        <Helper class="mt-1">
            <TextButton onclick={onEnterManuallyClick}>
                Enter coordinates manually
            </TextButton>
        </Helper>
    {:else}
        <div class="flex items-center gap-8 mt-4">
            <Label>
                Latitude
                <Input
                    type="number"
                    class="mt-1"
                    bind:value={config.address.search_centre.lat}
                />
            </Label>
            <Label>
                Longitude
                <Input
                    type="number"
                    class="mt-1"
                    bind:value={config.address.search_centre.lng}
                />
            </Label>
        </div>

        <TextButton class="mt-1" onclick={onClearClick}>
            Search for address
        </TextButton>
    {/if}
{/if}
