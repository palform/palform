<script lang="ts">
    import type { APICountryWithCallingCode } from "@palform/palform-typescript-openapi";
    import { Button, Dropdown, DropdownGroup, Search } from "flowbite-svelte";
    import { APIs } from "../../data/common";
    import { t } from "../../data/contexts/i18n";

    interface Props {
        value?: string;
        class?: string;
        disabled?: boolean;
        autoOpen?: boolean;
        allowedValues?: string[];
        onupdate?: (val: APICountryWithCallingCode) => void;
    }

    let {
        value = $bindable(""),
        class: className,
        onupdate,
        disabled,
        allowedValues,
        autoOpen = false,
    }: Props = $props();

    let countries = $state<APICountryWithCallingCode[] | undefined>(undefined);
    $effect(() => {
        allowedValues;
        APIs.countries.countriesListCallingCodes().then((resp) => {
            if (allowedValues !== undefined && allowedValues.length > 0) {
                countries = resp.data.filter((e) =>
                    allowedValues.includes(`+${e.calling_code}`)
                );
            } else {
                countries = resp.data;
            }
        });
    });

    let searchQuery = $state("");
    let filteredCountries = $derived(
        countries?.filter((c) => {
            return (
                c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                c.calling_code
                    .toString()
                    .startsWith(searchQuery.replaceAll("+", ""))
            );
        })
    );

    // svelte-ignore state_referenced_locally
    let dropdownOpen = $state(autoOpen);
    function onCallingCodeSelect(e: Event, country: APICountryWithCallingCode) {
        e.preventDefault();
        value = "+" + country.calling_code;
        onupdate?.(country);
        dropdownOpen = false;
    }
</script>

<Button color="light" class={className} {disabled}>
    {#if value === ""}
        {t("phone_choose_country")}
    {:else}
        {value}
    {/if}
</Button>
<Dropdown
    class="overflow-y-auto h-64 w-64 px-3 pb-3 text-sm"
    bind:isOpen={dropdownOpen}
>
    <div class="p-3">
        <Search
            size="sm"
            bind:value={searchQuery}
            placeholder={t("phone_search")}
            autofocus
        />
    </div>

    <DropdownGroup>
        {#each filteredCountries ?? [] as country (country.name)}
            <li>
                <button
                    class="w-full text-left rounded p-2 hover:bg-gray-50 dark:hover:bg-gray-600"
                    onclick={(e) => onCallingCodeSelect(e, country)}
                    type="button"
                >
                    <span class="block">
                        {country.flag_emoji}
                        <strong>(+{country.calling_code})</strong>
                    </span>
                    {country.name}
                </button>
            </li>
        {/each}
    </DropdownGroup>
</Dropdown>
