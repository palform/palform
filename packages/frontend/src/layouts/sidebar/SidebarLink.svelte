<script lang="ts">
    import { Link, useLocation } from "svelte-routing";
    import { isOrgRouteMatch } from "../../data/routing";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import type { IconProp } from "@fortawesome/fontawesome-svg-core";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faChevronDown } from "@fortawesome/free-solid-svg-icons";
    import { slide } from "svelte/transition";

    export let orgPath: string | undefined = undefined;
    export let icon: IconProp | undefined = undefined;
    export let dropdownLink: string | undefined = undefined;
    export let dropdownTitle: string | undefined = undefined;
    export let highlight = false;
    const location = useLocation();
    $: match =
        orgPath !== undefined && isOrgRouteMatch($location.pathname, orgPath);

    const orgCtx = getOrgContext();
</script>

<Link
    to={`/orgs/${$orgCtx.org.id}${dropdownLink ?? orgPath}`}
    class={`flex items-center justify-between py-2 px-4 rounded-lg ${highlight ? "bg-primary-200 dark:bg-primary-800 border border-primary-300 dark:border-primary-700" : ""} hover:bg-slate-200 dark:hover:bg-slate-600 ${match ? "bg-slate-200 dark:bg-slate-600" : ""} dark:text-gray-200`}
    on:click
>
    <span>
        {#if icon}
            <FontAwesomeIcon {icon} class="me-3 h-4 w-4" />
        {/if}
        {#if dropdownTitle === undefined}
            <slot />
        {:else}
            {dropdownTitle}
        {/if}
    </span>
    {#if dropdownTitle !== undefined}
        <span>
            <FontAwesomeIcon icon={faChevronDown} class={"text-slate-500"} />
        </span>
    {/if}
</Link>

{#if match && dropdownTitle !== undefined}
    <div transition:slide class="pl-4 space-y-2">
        <slot />
    </div>
{/if}
