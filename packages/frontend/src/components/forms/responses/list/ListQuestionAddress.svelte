<script lang="ts">
    import type { APIGenericAddress } from "@paltiverse/palform-client-js-extra-types/APIGenericAddress";
    import TextButton from "../../../TextButton.svelte";
    import { Modal } from "flowbite-svelte";
    import OpenLayersMap from "../../../map/OpenLayersMap.svelte";
    import type { APIGenericLocation } from "@paltiverse/palform-client-js-extra-types/APIGenericLocation";

    export let location: APIGenericLocation;
    export let address: APIGenericAddress;
    $: hasLocation = location.lat !== 0 && location.lng !== 0;
    let showModal = false;
</script>

<div class="text-gray-700 dark:text-gray-400">
    {#if address.line1}
        <p class="font-bold">{address.line1}</p>
    {/if}
    {#if address.line2}
        <p>{address.line2}</p>
    {/if}
    {#if address.locality}
        <p>{address.locality}</p>
    {/if}
    {#if address.postal_code}
        <p>{address.postal_code}</p>
    {/if}
    {#if address.iso_3166_alpha_1_code}
        <p>
            {address.iso_3166_alpha_1_code}
        </p>
    {/if}
</div>

{#if hasLocation}
    <TextButton class="mt-1" on:click={() => (showModal = true)}>
        Show on map
    </TextButton>

    <Modal title="View location" bind:open={showModal} outsideclose>
        <OpenLayersMap
            pinPoints={[[location.lat, location.lng]]}
            centerOn={[location.lat, location.lng]}
        />
    </Modal>
{/if}
