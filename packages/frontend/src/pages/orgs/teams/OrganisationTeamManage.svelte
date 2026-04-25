<script lang="ts">
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faCheckCircle } from "@fortawesome/free-solid-svg-icons";
    import { writable } from "svelte/store";
    import { setTeamCtx, type TeamContext } from "../../../data/contexts/team";
    import { Tabs } from "flowbite-svelte";
    import RoutedTabItem from "../../../components/tabs/RoutedTabItem.svelte";
    import { route } from "../../../router";
    import type { Snippet } from "svelte";

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    const teamId = $derived(route.params.teamId ?? "");
    const orgCtx = getOrgContext();
    let teamLoading = $state(true);
    let membersLoading = $state(true);
    let brandingsLoading = $state(true);

    const teamCtx = writable<TeamContext>(
        // @ts-expect-error we'll only render components that depend on these values once we know they are defined
        {}
    );
    setTeamCtx(teamCtx);

    APIs.orgTeams()
        .then((a) => a.organisationTeamsGet($orgCtx.org.id, teamId))
        .then((resp) => {
            $teamCtx.team = resp.data;
            teamLoading = false;
        });

    APIs.orgTeamMembers()
        .then((a) => a.organisationTeamMembersList($orgCtx.org.id, teamId))
        .then((resp) => {
            $teamCtx.members = resp.data;
            membersLoading = false;
        })
        .catch(showFailureToast);

    APIs.formBrandings()
        .then((a) => a.organisationTeamBrandingList($orgCtx.org.id, teamId))
        .then((resp) => {
            $teamCtx.brandings = resp.data;
            brandingsLoading = false;
        });
</script>

{#if !teamLoading && !membersLoading && !brandingsLoading && $teamCtx.team !== undefined && $teamCtx.members !== undefined && $teamCtx.brandings !== undefined}
    <MainTitle className="mb-4">
        {#if $teamCtx.team.is_default}
            <FontAwesomeIcon
                icon={faCheckCircle}
                class="text-primary-600 dark:text-primary-400 me-1"
            />
        {/if}
        {$teamCtx.team.name}
    </MainTitle>

    <Tabs classes={{ content: "p-0 h-0 m-0" }}>
        <RoutedTabItem title="Members" path="members" />
        <RoutedTabItem title="Branding" path="brandings" />
        <RoutedTabItem title="Settings" path="settings" />
    </Tabs>

    <div class="mt-4">
        {@render children?.()}
    </div>
{/if}
