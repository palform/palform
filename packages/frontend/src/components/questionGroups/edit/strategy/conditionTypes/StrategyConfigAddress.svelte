<script lang="ts">
    import type {
        APIQuestionConfigurationOneOf4Address,
        APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    } from "@paltiverse/palform-typescript-openapi";
    import {
        Button,
        ButtonGroup,
        InputAddon,
        Label,
        NumberInput,
        Select,
        Toggle,
    } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import { getCountryList } from "../../../../../data/billing/google";

    export let configuration: APIQuestionConfigurationOneOf4Address;

    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    let isNear = false;
    let nearLat = configuration.search_centre?.lat ?? 0;
    let nearLng = configuration.search_centre?.lng ?? 0;
    let nearRadius = 10;

    let isInCountry = false;
    let countryCode = "US";
    $: onSave = () => {
        dispatch("save", {
            Address: {
                near: isNear ? { lat: nearLat, lng: nearLng } : null,
                near_radius_km: isNear ? nearRadius : null,
                in_country: isInCountry ? countryCode : null,
            },
        });
    };

    let countries: { name: string; value: string }[] = [];
    getCountryList().then((resp) => (countries = resp));
</script>

<Toggle bind:checked={isNear}>Is near coordinate</Toggle>
{#if isNear}
    <div class="flex gap-4 mt-4 mb-4">
        <Label>
            Latitude
            <NumberInput
                type="number"
                size="sm"
                class="mt-1"
                bind:value={nearLat}
            />
        </Label>
        <Label>
            Longitude
            <NumberInput
                type="number"
                size="sm"
                class="mt-1"
                bind:value={nearLng}
            />
        </Label>
        <Label>
            Maximum radius
            <ButtonGroup class="mt-1">
                <NumberInput type="number" size="sm" bind:value={nearRadius} />
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

<Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
