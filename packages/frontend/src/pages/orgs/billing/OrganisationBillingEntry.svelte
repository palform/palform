<script lang="ts">
    import { billingEnabled } from "../../../data/billing/util";
    import OrganisationBilling from "./OrganisationBilling.svelte";
    import { navigate, route } from "../../../router";

    $effect(() => {
        if (!billingEnabled) {
            void navigate(`/orgs/${route.params.orgId}/`);
        }
    });
</script>

{#if billingEnabled}
    <OrganisationBilling />
{/if}
