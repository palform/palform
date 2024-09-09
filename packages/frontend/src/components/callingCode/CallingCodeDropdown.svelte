<script lang="ts">
    import type { APICountryWithCallingCode } from "@paltiverse/palform-typescript-openapi";
    import { Button, Dropdown, Search } from "flowbite-svelte";
    import { APIs } from "../../data/common";
    import { createEventDispatcher } from "svelte";
    import { t } from "../../data/contexts/i18n";

    export let value = "";
    const dispatch = createEventDispatcher<{
        update: APICountryWithCallingCode;
    }>();

    let countries: APICountryWithCallingCode[] | undefined = undefined;
    APIs.countries.countriesListCallingCodes().then((resp) => {
        countries = resp.data;
    });

    let searchQuery = "";
    $: filteredCountries = countries?.filter((c) => {
        return (
            c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
            c.calling_code
                .toString()
                .startsWith(searchQuery.replaceAll("+", ""))
        );
    });

    let dropdownOpen = false;
    $: onCallingCodeSelect = (e: Event, country: APICountryWithCallingCode) => {
        e.preventDefault();
        value = "+" + country.calling_code;
        dispatch("update", country);
        dropdownOpen = false;
    };
</script>

<Button color="light" class={$$props.class}>
    {#if value === ""}
        {t("phone_choose_country")}
    {:else}
        {value}
    {/if}
</Button>
<Dropdown
    class="overflow-y-auto h-64 w-64 px-3 pb-3 text-sm"
    bind:open={dropdownOpen}
>
    <div slot="header" class="p-3">
        <Search
            size="sm"
            bind:value={searchQuery}
            placeholder={t("phone_search")}
            autofocus
        />
    </div>

    {#each filteredCountries ?? [] as country (country.name)}
        <li>
            <button
                class="w-full text-left rounded p-2 hover:bg-gray-50 dark:hover:bg-gray-600"
                on:click={(e) => onCallingCodeSelect(e, country)}
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
</Dropdown>
