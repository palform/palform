<script lang="ts">
    import type { APIOrganisationAuthTeamMapping } from "@palform/palform-typescript-openapi";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import SkeletonPrimitive from "../../../components/SkeletonPrimitive.svelte";
    import TableContainer from "../../../components/tables/TableContainer.svelte";
    import {
        Button,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import TeamMappingNewModal from "../../../components/orgs/auth/mappings/TeamMappingNewModal.svelte";
    import TeamMappingRow from "../../../components/orgs/auth/mappings/TeamMappingRow.svelte";
    import { p } from "../../../router";

    const orgCtx = getOrgContext();
    let mappings: APIOrganisationAuthTeamMapping[] | undefined =
        $state(undefined);
    let loading = $state(false);
    APIs.orgAuthTeamMappings()
        .then((a) => a.organisationAuthConfigMappingsList($orgCtx.org.id))
        .then((resp) => {
            mappings = resp.data;
        })
        .catch((e) => showFailureToast(e))
        .finally(() => {
            loading = false;
        });

    let onNewMapping = $derived((e: APIOrganisationAuthTeamMapping) => {
        if (mappings === undefined) return;
        mappings = [e, ...mappings];
    });
    let onMappingDelete = $derived((id: string) => {
        if (mappings === undefined) return;
        mappings = mappings.filter((e) => e.id !== id);
    });
</script>

<Button
    class="mb-4"
    outline
    size="xs"
    href={p("/orgs/:orgId/settings/auth", {
        params: { orgId: $orgCtx.org.id },
    })}
>
    Back
</Button>

<MainTitle>Team mapping rules</MainTitle>

{#if loading}
    <SkeletonPrimitive className="p-4 mt-4 space-y-2">
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="30px" />
        <SkeletonPrimitive height="30px" />
        <SkeletonPrimitive height="30px" />
        <SkeletonPrimitive height="30px" />
        <SkeletonPrimitive height="30px" />
    </SkeletonPrimitive>
{/if}

{#if mappings}
    <TeamMappingNewModal class="mt-4" oncreate={onNewMapping} />

    {#if mappings.length > 0}
        <TableContainer class="mt-6">
            <Table divClass="">
                <TableHead>
                    <TableHeadCell>If groups field contains...</TableHeadCell>
                    <TableHeadCell>Then add to team</TableHeadCell>
                    <TableHeadCell>With role</TableHeadCell>
                    <TableHeadCell>
                        <span class="sr-only"> Actions </span>
                    </TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each mappings as mapping (mapping.id)}
                        <TeamMappingRow
                            {mapping}
                            ondelete={() => onMappingDelete(mapping.id)}
                        />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}
