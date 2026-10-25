<script lang="ts">
    import { p } from "../../../router";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { A, Button, P } from "flowbite-svelte";

    interface Props {
        class?: string;
    }
    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();
</script>

<CardGrid class={className}>
    <InductionStepCard title="Simple API access">
        Nearly anything you can do on the dashboard, you can do with Palform's
        API. It's easy to get started with our full-coverage OpenAPI specs and
        easy service account system.

        {#snippet footer()}
            <P class="mt-2">
                <A
                    href="https://docs.palform.app/api/getting_started"
                    target="_blank"
                >
                    Learn more
                </A>
            </P>
        {/snippet}
    </InductionStepCard>
    <InductionStepCard title="Granular permissions">
        Service accounts are just like real members of your organisation; they
        can have different permissions and even belong to multiple teams. You
        can give them as little or as much access as needed.
    </InductionStepCard>
    <InductionStepCard title="Create a service account">
        Service accounts and programmatic API access are available only on our
        upgraded plans at the moment.

        {#snippet footer()}
            <Button
                class="mt-3"
                href={p("/orgs/:orgId/settings/billing", {
                    params: { orgId: $orgCtx.org.id },
                })}
            >
                Upgrade now
            </Button>
        {/snippet}
    </InductionStepCard>
</CardGrid>
