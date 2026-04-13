<script lang="ts">
    import { isOrgRouteMatch } from "../../data/routing";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import type { IconProp } from "@fortawesome/fontawesome-svg-core";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faChevronDown } from "@fortawesome/free-solid-svg-icons";
    import { slide } from "svelte/transition";
    import { route } from "../../router";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        orgPath?: string | undefined;
        icon?: IconProp | undefined;
        dropdownTitle?: string | undefined;
        highlight?: boolean;
        activationLevel?: number;
        children?: import("svelte").Snippet;
        onclick?: MouseEventHandler<HTMLAnchorElement>;
    }

    let {
        orgPath = undefined,
        icon = undefined,
        dropdownTitle = undefined,
        highlight = false,
        activationLevel: _activationLevel,
        children,
        onclick,
    }: Props = $props();

    let activationLevel = $derived.by(() => {
        if (_activationLevel !== undefined) return _activationLevel;
        if (orgPath === undefined) return 1;
        return orgPath.split("/").length - 1;
    });

    let match = $derived(
        orgPath !== undefined &&
            isOrgRouteMatch(route.pathname, orgPath, activationLevel)
    );

    const orgCtx = getOrgContext();
</script>

<a
    href={`/orgs/${$orgCtx.org.id}${orgPath}`}
    class={`flex items-center justify-between py-2 px-4 rounded-lg ${highlight ? "bg-primary-200 dark:bg-primary-800 border border-primary-300 dark:border-primary-700" : ""} hover:bg-slate-200 dark:hover:bg-slate-600 ${match ? "bg-slate-200 dark:bg-slate-600" : ""} dark:text-gray-200`}
    {onclick}
>
    <span>
        {#if icon}
            <FontAwesomeIcon {icon} class="me-3 h-4 w-4" />
        {/if}
        {#if dropdownTitle === undefined}
            {@render children?.()}
        {:else}
            {dropdownTitle}
        {/if}
    </span>
    {#if dropdownTitle !== undefined}
        <span>
            <FontAwesomeIcon icon={faChevronDown} class={"text-slate-500"} />
        </span>
    {/if}
</a>

{#if match && dropdownTitle !== undefined}
    <div transition:slide class="pl-4 space-y-2">
        {@render children?.()}
    </div>
{/if}
