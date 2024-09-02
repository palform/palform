<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import InfoText from "../type/InfoText.svelte";
    import SectionHeading from "../type/SectionHeading.svelte";
    import FormCard from "./FormCard.svelte";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { navigateEvent } from "../../utils/navigate";

    export let teamId: string;
    const orgCtx = getOrgContext();
    $: team = $orgCtx.myTeams.find((e) => e.team_id === teamId);
    $: forms = $orgCtx.forms.filter((e) => e.team_id === teamId);
</script>

{#if team !== undefined}
    <SectionHeading>
        {team.name}
    </SectionHeading>

    <Button
        outline
        color="light"
        size="xs"
        class="mt-2 mb-4"
        href={`/orgs/${$orgCtx.org.id}/forms/new/${teamId}`}
        on:click={navigateEvent}
    >
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Create form
    </Button>

    {#if forms.length === 0}
        <InfoText>No forms yet...</InfoText>
    {:else}
        <div class="grid lg:grid-cols-3 gap-4">
            {#each forms as form (form.id)}
                <FormCard {form} orgId={$orgCtx.org.id} />
            {/each}
        </div>
    {/if}
{/if}
