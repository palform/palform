<script lang="ts">
    import type { APIUserKeyWithIdentity } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import TableContainer from "../../tables/TableContainer.svelte";
    import {
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import OrganisationKeyTableRow from "./OrganisationKeyTableRow.svelte";

    const orgCtx = getOrgContext();
    let loading = true;
    let keys: APIUserKeyWithIdentity[] | null = null;

    APIs.orgKeys()
        .then((a) => a.orgKeysList($orgCtx.org.id))
        .then((resp) => {
            keys = resp.data;
            loading = false;
        })
        .catch(showFailureToast);

    $: onKeyDelete = (id: string) => {
        if (!keys) return;
        keys = keys.filter((e) => e.id !== id);
    };
</script>

{#if loading}
    <SkeletonPrimitive height="50px" />
    <SkeletonPrimitive height="40px" className="mt-1" />
    <SkeletonPrimitive height="40px" className="mt-1" />
    <SkeletonPrimitive height="40px" className="mt-1" />
    <SkeletonPrimitive height="40px" className="mt-1" />
    <SkeletonPrimitive height="40px" className="mt-1" />
    <SkeletonPrimitive height="40px" className="mt-1" />
    <SkeletonPrimitive height="40px" className="mt-1" />
{:else if keys !== null}
    <TableContainer>
        <Table divClass="">
            <TableHead>
                <TableHeadCell>Owner</TableHeadCell>
                <TableHeadCell>Fingerprint</TableHeadCell>
                <TableHeadCell>Created</TableHeadCell>
                <TableHeadCell>Expires</TableHeadCell>
                <TableHeadCell>
                    <span class="sr-only">Actions</span>
                </TableHeadCell>
            </TableHead>
            <TableBody>
                {#each keys as key (key.id)}
                    <OrganisationKeyTableRow
                        {key}
                        on:delete={() => onKeyDelete(key.id)}
                    />
                {/each}
            </TableBody>
        </Table>
    </TableContainer>
{/if}
