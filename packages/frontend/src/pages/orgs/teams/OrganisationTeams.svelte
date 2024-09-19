<script lang="ts">
    import type { APIOrganisationTeam } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import CardBox from "../../../components/cardBox/CardBox.svelte";
    import CardBoxTitle from "../../../components/cardBox/CardBoxTitle.svelte";
    import CardBoxSubtitle from "../../../components/cardBox/CardBoxSubtitle.svelte";
    import { Button, Tooltip } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faArrowRight,
        faCheckCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import NewTeamModal from "../../../components/teams/NewTeamModal.svelte";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    const orgCtx = getOrgContext();

    let loading = true;
    let teams: APIOrganisationTeam[] = [];

    $: APIs.orgTeams()
        .then((a) => a.organisationTeamsList($orgCtx.org.id))
        .then((resp) => {
            teams = resp.data;
            loading = false;
        });
</script>

<NewTeamModal class="mb-4" />

{#if !loading && teams.length === 1}
    <CardGrid class="mb-8">
        <InductionStepCard title="Group users into teams">
            Manage your organisation using teams, each owning their own forms
            and reusable branding configurations.
        </InductionStepCard>
        <InductionStepCard title="...or keep things simple">
            You don't have to use teams! If you don't need complex access
            management, you can simply use your <em>default team</em>.
        </InductionStepCard>
        <InductionStepCard title="Assign members easily">
            Bulk-assign members to teams in a few easy clicks. Everyone is
            automatically added to the default team.
        </InductionStepCard>
    </CardGrid>
{/if}

<div class="grid lg:grid-cols-3 xl:grid-cols-4 gap-4">
    {#each teams as team (team.id)}
        <CardBox>
            <CardBoxTitle>
                {#if team.is_default}
                    <FontAwesomeIcon icon={faCheckCircle} />
                    <Tooltip>This is your default team.</Tooltip>
                {/if}
                {team.name}
            </CardBoxTitle>
            <CardBoxSubtitle>
                {team.num_members} member{team.num_members === 1 ? "" : "s"}
            </CardBoxSubtitle>
            <Button
                outline
                class="mt-2"
                href={`/orgs/${$orgCtx.org.id}/settings/teams/${team.id}/members`}
                on:click={navigateEvent}
            >
                Manage
                <FontAwesomeIcon icon={faArrowRight} class="ms-2" />
            </Button>
        </CardBox>
    {/each}
</div>
