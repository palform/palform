<script lang="ts">
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faCheckCircle } from "@fortawesome/free-solid-svg-icons";
    import { writable } from "svelte/store";
    import { setTeamCtx, type TeamContext } from "../../../data/contexts/team";
    import { Route, Router } from "svelte-routing";
    import TeamMembers from "./TeamMembers.svelte";
    import { Tabs } from "flowbite-svelte";
    import RoutedTabItem from "../../../components/tabs/RoutedTabItem.svelte";
    import TeamBrandings from "./TeamBrandings.svelte";
    import TeamSettings from "./TeamSettings.svelte";

    export let teamId: string;
    const orgCtx = getOrgContext();
    let teamLoading = true;
    let membersLoading = true;
    let brandingsLoading = true;

    const teamCtx = writable<TeamContext>(
        // @ts-expect-error we'll only render components that depend on these values once we know they are defined
        {},
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

    <Router>
        <Tabs contentClass="mt-4">
            <RoutedTabItem title="Members" path="members" />
            <RoutedTabItem title="Branding" path="brandings" />
            <RoutedTabItem title="Settings" path="settings" />
        </Tabs>

        <Route path="/members" component={TeamMembers} />
        <Route path="/brandings" component={TeamBrandings} />
        <Route path="/settings" component={TeamSettings} />
    </Router>
{/if}
