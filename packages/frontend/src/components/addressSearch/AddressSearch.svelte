<script lang="ts">
    import type { APIGenericAddress } from "@paltiverse/palform-client-js-extra-types/APIGenericAddress";
    import { createEventDispatcher, onMount } from "svelte";
    import {
        getMapboxAPI,
        parseAddressComponents,
        parseResponseGeometry,
        type MapboxAPIType,
    } from "../../data/billing/google";
    import { Input, Label } from "flowbite-svelte";
    import InfoText from "../type/InfoText.svelte";
    import type { APIGenericLocation } from "@paltiverse/palform-client-js-extra-types/APIGenericLocation";
    import type {
        AddressAutofillSuggestion,
        LngLatLike,
    } from "@mapbox/search-js-core";
    import { t } from "../../data/contexts/i18n";

    export let countryCode: string | undefined = undefined;
    export let locationBias: LngLatLike | string | undefined = undefined;
    const dispatch = createEventDispatcher<{
        select: { address: APIGenericAddress; location: APIGenericLocation };
    }>();

    let api: MapboxAPIType | undefined = undefined;
    undefined;

    onMount(async () => {
        api = getMapboxAPI();
    });

    let query = "";
    let suggestions: AddressAutofillSuggestion[] = [];

    $: makeRequest = async () => {
        if (!api) return;
        if (query.trim().length === 0) {
            suggestions = [];
            return;
        }

        const resp = await api.suggest(query.trim(), {
            country: countryCode,
            proximity: locationBias,
        });

        suggestions = resp.suggestions;
    };

    let timeout: number | undefined = undefined;
    $: onChange = () => {
        if (timeout) {
            clearTimeout(timeout);
            timeout = undefined;
        }
        timeout = setTimeout(() => {
            makeRequest();
        }, 100) as unknown as number;
    };

    const onAddressClick = async (suggestion: AddressAutofillSuggestion) => {
        if (!api) return;
        const resp = await api.retrieve(suggestion);

        const parsedComponents = parseAddressComponents(suggestion);
        if (!parsedComponents) return;
        const parsedLocation = parseResponseGeometry(resp);
        if (!parsedLocation) return;

        dispatch("select", {
            address: parsedComponents,
            location: parsedLocation,
        });
    };
</script>

<div class={$$props.class}>
    <Label>
        {t("address_search")}
        <Input
            bind:value={query}
            class="mt-1"
            placeholder={t("address_start_typing")}
            on:input={onChange}
        />
    </Label>

    {#if suggestions.length > 0}
        <InfoText class="mt-4">{t("address_select")}</InfoText>
        <div
            class="mt-4 rounded-lg border border-gray-300 dark:border-slate-600 overflow-hidden"
        >
            {#each suggestions as suggestion (suggestion.action.id)}
                <button
                    class="w-full px-4 py-2 bg-gray-50 dark:bg-slate-800 odd:bg-gray-100 dark:odd:bg-slate-900 hover:bg-gray-200 dark:hover:bg-gray-700 text-left dark:text-slate-300"
                    on:click={() => onAddressClick(suggestion)}
                    type="button"
                >
                    {suggestion.full_address}
                </button>
            {/each}
        </div>
    {/if}
</div>
