<script lang="ts">
    import { Button } from "flowbite-svelte";
    import CardGrid from "../../components/CardGrid.svelte";
    import InductionStepCard from "../../components/induction/InductionStepCard.svelte";
    import { isEntitled } from "../../data/billing/entitlement";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { navigate } from "svelte-routing";
    import AuditLogBrowser from "../../components/orgs/audit/AuditLogBrowser.svelte";

    const entitled = isEntitled("audit");
    const orgCtx = getOrgContext();
    $: onContinueClick = () =>
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
</script>

{#if !$entitled}
    <CardGrid>
        <InductionStepCard title="See everything">
            Audit logs give you complete oversight over everything happening in
            your organisation's Palform.
        </InductionStepCard>
        <InductionStepCard title="Privacy friendly">
            Our logs are stored in a secure database and deleted automatically
            after 7 days. Only organisation admins can view them.
        </InductionStepCard>
        <InductionStepCard title="Get started">
            To view your audit logs, please upgrade your plan.
            <Button slot="footer" class="mt-2" on:click={onContinueClick}>
                Continue
            </Button>
        </InductionStepCard>
    </CardGrid>
{:else}
    <AuditLogBrowser />
{/if}
