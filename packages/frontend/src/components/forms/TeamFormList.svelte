<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import InfoText from "../type/InfoText.svelte";
    import SectionHeading from "../type/SectionHeading.svelte";
    import FormCard from "./FormCard.svelte";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { p } from "../../router";

    interface Props {
        teamId: string;
    }

    let { teamId }: Props = $props();
    const orgCtx = getOrgContext();
    let team = $derived($orgCtx.myTeams.find((e) => e.team_id === teamId));
    let forms = $derived($orgCtx.forms.filter((e) => e.team_id === teamId));
</script>

{#if team !== undefined}
    <SectionHeading>
        {team.name}
    </SectionHeading>

    <Button
        color="light"
        size="xs"
        class="mt-2 mb-4"
        href={p("/orgs/:orgId/forms/templates", {
            params: { orgId: $orgCtx.org.id },
        })}
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
