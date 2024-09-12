<script lang="ts">
    import { writable } from "svelte/store";
    import {
        reloadGlobalAlert,
        reloadInduction,
        setOrgContext,
        type OrgContext,
    } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import SidebarLink from "./SidebarLink.svelte";
    import {
        faArrowLeft,
        faBars,
        faBuilding,
        faCog,
        faHome,
        faInfoCircle,
        faPerson,
        faPlus,
    } from "@fortawesome/free-solid-svg-icons";
    import { Link, navigate } from "svelte-routing";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import SidebarSeparator from "./SidebarSeparator.svelte";
    import SidebarFormEntry from "./SidebarFormEntry.svelte";
    import GlobalWarning from "./GlobalWarning.svelte";
    import { billingEnabled } from "../../data/billing/util";
    import OrganisationLoading from "./OrganisationLoading.svelte";
    import { getOrgSubdomain, signOut } from "../../data/auth";
    import TextButton from "../../components/TextButton.svelte";
    import { Button } from "flowbite-svelte";
    import { onMount } from "svelte";
    import { enable_debug_hook_js } from "@paltiverse/palform-client-common";

    enable_debug_hook_js();

    export let orgId: string;
    const orgCtx = writable<OrgContext>(
        // @ts-expect-error We'll only load components that depend on this context once we know that all attributes are initialised
        {}
    );
    setOrgContext(orgCtx);
    APIs.orgs()
        .then((a) => a.orgsGet(orgId))
        .then((resp) => {
            $orgCtx.org = resp.data;
            reloadGlobalAlert(orgCtx);
            reloadInduction(orgCtx);
        });

    APIs.orgTeams()
        .then((a) => a.organisationTeamsListMy(orgId))
        .then((resp) => {
            $orgCtx.myTeams = resp.data;
        });

    APIs.forms()
        .then((a) => a.formsList(orgId))
        .then((resp) => {
            $orgCtx.forms = resp.data;
        });

    APIs.orgMembers()
        .then((a) => a.organisationMembersAmIAdmin(orgId))
        .then((resp) => {
            $orgCtx.amIAdmin = resp.data;
        });

    if (billingEnabled) {
        APIs.billingEntitlements()
            .then((a) => a.billingEntitlementList(orgId))
            .then((resp) => {
                $orgCtx.entitlements = resp.data;
            });
    }

    const isOnSubdomain = getOrgSubdomain() !== null;
    const onSignOutClick = async (e: Event) => {
        e.preventDefault();
        await signOut();
        navigate("/auth/login");
    };

    const closeSidebar = () => {
        mobileSidebarOpen = false;
    };

    let screenWidth = window.innerWidth;
    $: sidebarIsMobile = screenWidth <= 768;
    let mobileSidebarOpen = false;
    onMount(() => {
        const listener = () => {
            screenWidth = window.innerWidth;
        };
        window.addEventListener("resize", listener);

        return () => {
            window.removeEventListener("resize", listener);
        };
    });

    let orgIsHovered = false;
    $: onNewFormClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/forms/new`);
        closeSidebar();
    };
</script>

{#if $orgCtx === undefined || $orgCtx.org === undefined || $orgCtx.forms === undefined || $orgCtx.myTeams === undefined || $orgCtx.induction === undefined || $orgCtx.amIAdmin === undefined}
    <OrganisationLoading />
{/if}

{#if $orgCtx !== undefined && $orgCtx.org !== undefined && $orgCtx.myTeams !== undefined && $orgCtx.forms !== undefined && $orgCtx.induction !== undefined && $orgCtx.amIAdmin !== undefined}
    <div
        class="flex overflow-hidden h-screen w-full bg-slate-50/50 dark:bg-slate-900"
    >
        <div
            class={`h-screen bg-slate-100 dark:bg-slate-800 flex flex-col gap-2 transition-[max-width] w-72 overflow-hidden ${!sidebarIsMobile || mobileSidebarOpen ? "max-w-72 p-3" : "max-w-0 p-0"}`}
        >
            {#if !isOnSubdomain}
                <p
                    class="text-sm mb-4 text-gray-600 dark:text-gray-400"
                    on:mouseover={() => (orgIsHovered = true)}
                    on:focus={() => (orgIsHovered = true)}
                    on:mouseout={() => (orgIsHovered = false)}
                    on:blur={() => (orgIsHovered = false)}
                >
                    <Link to="/" class="hover:underline">
                        {#if orgIsHovered}
                            <FontAwesomeIcon icon={faArrowLeft} />
                            All orgs
                        {:else}
                            <FontAwesomeIcon icon={faBuilding} />
                            {$orgCtx.org.display_name}
                        {/if}
                    </Link>
                </p>
            {/if}

            <SidebarLink orgPath="/" icon={faHome} on:click={closeSidebar}>
                Home
            </SidebarLink>
            {#if !$orgCtx.induction.induction_complete}
                <SidebarLink
                    orgPath="/induction"
                    icon={faInfoCircle}
                    highlight
                    on:click={closeSidebar}
                >
                    Getting started
                </SidebarLink>
            {/if}
            <SidebarLink
                orgPath="/user"
                dropdownLink="/user/keys"
                dropdownTitle="My account"
                icon={faPerson}
            >
                <SidebarLink orgPath="/user/keys" on:click={closeSidebar}
                    >Keys</SidebarLink
                >
                <SidebarLink orgPath="/user/settings" on:click={closeSidebar}
                    >Settings</SidebarLink
                >
            </SidebarLink>
            <SidebarLink
                orgPath="/settings"
                dropdownLink="/settings/teams"
                dropdownTitle="Organisation"
                icon={faCog}
            >
                <SidebarLink orgPath="/settings/teams" on:click={closeSidebar}
                    >Teams</SidebarLink
                >
                {#if $orgCtx.amIAdmin}
                    <SidebarLink
                        orgPath="/settings/members"
                        on:click={closeSidebar}
                    >
                        Members
                    </SidebarLink>
                    <SidebarLink
                        orgPath="/settings/keys"
                        on:click={closeSidebar}
                    >
                        Manage keys
                    </SidebarLink>
                    <SidebarLink
                        orgPath="/settings/audit"
                        on:click={closeSidebar}>Audit logs</SidebarLink
                    >
                    <SidebarLink
                        orgPath="/settings/subdomain"
                        on:click={closeSidebar}
                    >
                        Subdomain
                    </SidebarLink>
                    <SidebarLink
                        orgPath="/settings/auth"
                        on:click={closeSidebar}
                    >
                        Authentication
                    </SidebarLink>
                    {#if billingEnabled}
                        <SidebarLink
                            orgPath="/settings/billing"
                            on:click={closeSidebar}
                        >
                            Billing
                        </SidebarLink>
                    {/if}
                    <SidebarLink orgPath="/settings/org" on:click={closeSidebar}
                        >Settings</SidebarLink
                    >
                {/if}
            </SidebarLink>

            <div class="mb-2">
                <SidebarSeparator />
                <div class="flex gap-4 w-full items-center">
                    <p
                        class="text-gray-600 dark:text-gray-400 tracking-wider text-sm flex-1"
                    >
                        Forms
                    </p>
                    <Button size="xs" outline on:click={onNewFormClick}>
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Create new
                    </Button>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto space-y-2">
                {#if $orgCtx.forms.length > 0}
                    {#each $orgCtx.forms as form (form.id)}
                        <SidebarFormEntry {form} on:click={closeSidebar} />
                    {/each}
                {/if}
            </div>

            <div class="pt-2">
                <TextButton on:click={onSignOutClick}>Sign out</TextButton>
            </div>
        </div>
        <div class="flex-1 overflow-x-auto">
            {#if sidebarIsMobile && !mobileSidebarOpen}
                <div class="fixed top-4 left-4">
                    <button
                        class="px-2 py-1 bg-primary-100 rounded-md"
                        on:click={() => (mobileSidebarOpen = true)}
                    >
                        <FontAwesomeIcon icon={faBars} />
                    </button>
                </div>
            {/if}

            <GlobalWarning />
            <div
                class="w-full overflow-y-auto px-4 md:px-14 lg:px-32 pt-14 md:pt-8 pb-10"
            >
                <slot />
            </div>
        </div>
    </div>
{:else}
    <OrganisationLoading />
{/if}
