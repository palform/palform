<script lang="ts">
    import { Alert, Button } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import CardGrid from "../../components/CardGrid.svelte";
    import InductionStepCard from "../../components/induction/InductionStepCard.svelte";
    import {
        faFileShield,
        faLock,
        faPeopleGroup,
    } from "@fortawesome/free-solid-svg-icons";
    import { isEntitled } from "../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../../components/billing/entitlement/MissingEntitlementTooltip.svelte";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    const orgCtx = getOrgContext();
    const multiMemberEntitled = isEntitled("user_count", true);
</script>

{#if $orgCtx.induction.induction_complete}
    <Alert color="green" border class="mt-8">
        Thanks for completing the induction! We hope you have lots of fun using
        Palform.

        <Button
            class="block mt-2 w-fit"
            size="sm"
            color="green"
            href={`/orgs/${$orgCtx.org.id}`}
            on:click={navigateEvent}
        >
            Continue
        </Button>
    </Alert>
{:else}
    <CardGrid class="mt-8">
        <InductionStepCard
            title="Create a key"
            href={`/orgs/${$orgCtx.org.id}/user/keys`}
            icon={faLock}
            checked={$orgCtx.induction.key_created}
        >
            Keys power our end-to-end encryption. Everyone needs their own key
            to receive and decrypt form responses.
        </InductionStepCard>
        {#if $orgCtx.induction.can_create_invite && !$orgCtx.org.uses_oidc}
            <InductionStepCard
                title="Invite your team"
                href={`/orgs/${$orgCtx.org.id}/settings/members/invite`}
                icon={faPeopleGroup}
                checked={$orgCtx.induction.invite_created}
                disabled={!$multiMemberEntitled}
            >
                Everything is better with friends — even forms! Set permissions
                and generate invite links from one easy dashboard.
            </InductionStepCard>
            <MissingEntitlementTooltip
                placement="bottom"
                key="user_count"
                multi
            />
        {/if}
        <InductionStepCard
            title="Create a form"
            href={`/orgs/${$orgCtx.org.id}/forms/new`}
            icon={faFileShield}
            checked={$orgCtx.induction.form_created}
        >
            Use our super-easy online editor to make your first end-to-end
            encrypted form in just a few clicks.
        </InductionStepCard>
    </CardGrid>
{/if}
