<script lang="ts">
    import { Button } from "flowbite-svelte";
    import BrandingContextProvider from "../../../components/teams/brandings/BrandingContextProvider.svelte";
    import BrandingPreviewCard from "../../../components/teams/brandings/BrandingPreviewCard.svelte";
    import { getTeamCtx } from "../../../data/contexts/team";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import BrandingConfigModal from "../../../components/teams/brandings/BrandingConfigModal.svelte";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../../../components/billing/entitlement/MissingEntitlementTooltip.svelte";

    const teamCtx = getTeamCtx();
    const brandingEntitled = isEntitled("branding_count");
    const multiBrandingEntitled = isEntitled("branding_count", true);

    let showNewModal = false;
</script>

<Button
    class="mb-4"
    outline
    on:click={() => (showNewModal = true)}
    disabled={!$brandingEntitled ||
        ($teamCtx.brandings.length > 1 && $multiBrandingEntitled)}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Create new branding scheme
</Button>
<MissingEntitlementTooltip
    multi={$teamCtx.brandings.length > 1}
    key="branding_count"
/>

{#if $teamCtx.brandings.length === 0}
    <CardGrid>
        <InductionStepCard title="Personalise your forms">
            Each team has a set of "branding schemes": these customise the
            appearance and feel of its forms.
        </InductionStepCard>
        <InductionStepCard title="Reusable configurations">
            Apply a single branding scheme to many forms, then update it from
            one central place if you need to.
        </InductionStepCard>
        <InductionStepCard title="Or leave it to us!">
            Palforms look great even without custom branding. Your forms will
            get our default styling if you don't assign a scheme to them.
        </InductionStepCard>
    </CardGrid>
{/if}

<BrandingConfigModal bind:modalOpen={showNewModal} />

<div class="grid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
    {#each $teamCtx.brandings as branding (branding.id)}
        <BrandingContextProvider ctx={branding}>
            <BrandingPreviewCard {branding} />
        </BrandingContextProvider>
    {/each}
</div>
