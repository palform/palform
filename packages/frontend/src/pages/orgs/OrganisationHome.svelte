<script lang="ts">
    import MainTitle from "../../layouts/MainTitle.svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import SectionSeparator from "../../components/type/SectionSeparator.svelte";
    import TeamFormList from "../../components/forms/TeamFormList.svelte";
    import { DateTime } from "luxon";
    import { Alert, Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import OrganisationLatestUpdates from "../../components/orgs/dashboard/OrganisationLatestUpdates.svelte";
    import { p } from "../../router";

    const orgCtx = getOrgContext();
    const currentTime = DateTime.now();
    let greeting = $derived.by(() => {
        if (currentTime.hour < 6) {
            return "Hey, night owl";
        } else if (currentTime.hour < 12) {
            return "Good morning";
        } else if (currentTime.hour < 18) {
            return "Good afternoon";
        } else {
            return "Good evening";
        }
    });
</script>

<MainTitle className="font-bold">
    {greeting}!
</MainTitle>
<p class="text-lg dark:text-slate-300">
    Welcome to {$orgCtx.org.display_name} 👋
</p>

{#if !$orgCtx.induction.induction_complete}
    <Alert class="mt-4" border>
        <h2 class="text-lg font-medium">
            You've almost finished setting up Palform!
        </h2>
        <p>
            Just a few more simple steps to go before you can start collecting
            super secure form responses.
        </p>
        <Button
            class="mt-2"
            size="lg"
            href={p("/orgs/:orgId/induction", {
                params: { orgId: $orgCtx.org.id },
            })}
        >
            Continue setup
            <FontAwesomeIcon class="ms-2" icon={faArrowRight} />
        </Button>

        <p class="text-xs mt-2">
            We'll hide this message 7 days after creating your organisation.
        </p>
    </Alert>
{/if}

<section class="mt-2">
    <OrganisationLatestUpdates />
</section>

<div class="space-y-6 mt-4">
    <SectionSeparator />
    {#each $orgCtx.myTeams as team (team.team_id)}
        <section>
            <TeamFormList teamId={team.team_id} />
        </section>

        <SectionSeparator />
    {/each}
</div>
