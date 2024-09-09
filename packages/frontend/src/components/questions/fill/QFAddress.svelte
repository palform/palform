<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import type { APIQuestionConfigurationOneOf4 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import {
        fillSendStore,
        setQuestionValue,
        sGetAddress,
        sIsNonEmpty,
    } from "../../../data/contexts/fill";
    import AddressSearch from "../../addressSearch/AddressSearch.svelte";
    import { Input, Label, Select } from "flowbite-svelte";
    import type { APIGenericAddress } from "@paltiverse/palform-client-js-extra-types/APIGenericAddress";
    import type { APIGenericLocation } from "@paltiverse/palform-client-js-extra-types/APIGenericLocation";
    import { getCountryList } from "../../../data/billing/google";
    import TextButton from "../../TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faRotateRight } from "@fortawesome/free-solid-svg-icons";
    import { t } from "../../../data/contexts/i18n";

    const dispatch = createEventDispatcher<{ change: undefined }>();

    export let id: string;
    export let config: APIQuestionConfigurationOneOf4;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue ? sGetAddress(currentValue) : null;
    $: empty = currentValue ? !sIsNonEmpty(currentValue) : true;

    $: onAddressClick = (
        e: CustomEvent<{
            address: APIGenericAddress;
            location: APIGenericLocation;
        }>
    ) => {
        setQuestionValue(id, {
            Address: {
                address: e.detail.address,
                point: e.detail.location,
            },
        });
        dispatch("change");
    };
    $: onAddressChange = (
        e: Event,
        change: (t: string, a: APIGenericAddress) => APIGenericAddress
    ) => {
        if (currentValue === undefined || value === null) return;
        const t = (e.target as HTMLInputElement).value;
        setQuestionValue(id, {
            Address: {
                address: change(t, value.address),
                point: value.point,
            },
        });
        dispatch("change");
    };
    $: onClearClick = () => {
        setQuestionValue(id, {
            Address: {
                address: {
                    line1: null,
                    line2: null,
                    locality: null,
                    postal_code: null,
                    iso_3166_alpha_1_code: null,
                },
                point: { lat: 0, lng: 0 },
            },
        });
        dispatch("change");
    };

    let countries: { name: string; value: string }[] | undefined = undefined;
    getCountryList().then((resp) => (countries = resp));
</script>

{#if empty}
    <AddressSearch
        on:select={onAddressClick}
        disabled={$fillSendStore?.loading || currentValue === undefined}
        locationBias={config.address.search_centre
            ? {
                  lat: config.address.search_centre.lat,
                  lon: config.address.search_centre.lng,
              }
            : "ip"}
    />
{:else}
    <div class="w-full flex items-start gap-4 xl:gap-8">
        <div class="flex-1 space-y-4">
            <Label>
                {t("address_line_1")}
                <Input
                    class="mt-1"
                    value={value?.address.line1}
                    on:input={(e) =>
                        onAddressChange(e, (t, a) => ({ ...a, line1: t }))}
                    disabled={$fillSendStore?.loading}
                />
            </Label>
            <Label>
                {t("address_line_2")}
                <Input
                    class="mt-1"
                    value={value?.address.line2}
                    on:input={(e) =>
                        onAddressChange(e, (t, a) => ({ ...a, line2: t }))}
                    disabled={$fillSendStore?.loading}
                />
            </Label>
            <Label>
                {t("address_zip_code")}
                <Input
                    class="mt-1"
                    value={value?.address.postal_code}
                    on:input={(e) =>
                        onAddressChange(e, (t, a) => ({
                            ...a,
                            postal_code: t,
                        }))}
                    disabled={$fillSendStore?.loading}
                />
            </Label>
        </div>
        <div class="flex-1 space-y-4">
            <Label>
                {t("address_city")}
                <Input
                    class="mt-1"
                    value={value?.address.locality}
                    on:input={(e) =>
                        onAddressChange(e, (t, a) => ({ ...a, locality: t }))}
                    disabled={$fillSendStore?.loading}
                />
            </Label>
            <Label>
                {t("address_country")}
                <Select
                    class="mt-1"
                    items={countries ?? []}
                    value={value?.address.iso_3166_alpha_1_code}
                    on:change={(e) =>
                        onAddressChange(e, (t, a) => ({
                            ...a,
                            iso_3166_alpha_1_code: t,
                        }))}
                    disabled={$fillSendStore?.loading ||
                        countries === undefined}
                />
            </Label>
        </div>
    </div>

    <TextButton
        class="mt-4"
        on:click={onClearClick}
        disabled={$fillSendStore?.loading}
    >
        <FontAwesomeIcon icon={faRotateRight} />
        {t("address_new_search")}
    </TextButton>
{/if}
