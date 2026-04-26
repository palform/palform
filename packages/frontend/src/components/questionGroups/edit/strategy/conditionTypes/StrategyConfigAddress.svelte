<script lang="ts">
    import type { ConfigAddressAddress } from "@palform/palform-typescript-openapi";
    import {
        Button,
        ButtonGroup,
        Input,
        InputAddon,
        Label,
        Select,
        Toggle,
    } from "flowbite-svelte";
    import { getCountryList } from "../../../../../data/billing/google";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {
        configuration: ConfigAddressAddress;
    }

    let { configuration, onsave }: Props = $props();

    let isNear = $state(false);
    // svelte-ignore state_referenced_locally
    let nearLat = $state(configuration.search_centre?.lat ?? 0);
    // svelte-ignore state_referenced_locally
    let nearLng = $state(configuration.search_centre?.lng ?? 0);
    let nearRadius = $state(10);

    let isInCountry = $state(false);
    let countryCode = $state("US");
    let onSave = $derived(() => {
        onsave({
            Address: {
                near: isNear ? { lat: nearLat, lng: nearLng } : null,
                near_radius_km: isNear ? nearRadius : null,
                in_country: isInCountry ? countryCode : null,
            },
        });
    });

    let countries: { name: string; value: string }[] = $state([]);
    getCountryList().then((resp) => (countries = resp));
</script>

<Toggle bind:checked={isNear}>Is near coordinate</Toggle>
{#if isNear}
    <div class="flex gap-4 mt-4 mb-4">
        <Label>
            Latitude
            <Input type="number" size="sm" class="mt-1" bind:value={nearLat} />
        </Label>
        <Label>
            Longitude
            <Input type="number" size="sm" class="mt-1" bind:value={nearLng} />
        </Label>
        <Label>
            Maximum radius
            <ButtonGroup class="mt-1">
                <Input type="number" size="sm" bind:value={nearRadius} />
                <InputAddon>km</InputAddon>
            </ButtonGroup>
        </Label>
    </div>
{/if}

<Toggle class="mt-4" bind:checked={isInCountry}>Is in country</Toggle>
{#if isInCountry}
    <Label class="mt-3">
        Country
        <Select items={countries} bind:value={countryCode} class="mt-1" />
    </Label>
{/if}

<Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
