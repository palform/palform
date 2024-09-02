<script lang="ts">
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import type { APISubmissionCountPerForm } from "@paltiverse/palform-typescript-openapi";
    import { Link } from "svelte-routing";

    export let data: APISubmissionCountPerForm;
    const formCtx = getFormCtx(data.form_id);
    const orgCtx = getOrgContext();

    $: team = $orgCtx.myTeams.find((e) => e.team_id === data.team_id);
</script>

{#if $formCtx && team}
    <TableBodyRow>
        <TableBodyCell>
            <Link to={`/orgs/${$orgCtx.org.id}/forms/${data.form_id}/`}>
                <span class="block text-gray-500">
                    {team.name}
                </span>
                {$formCtx.title}
            </Link>
        </TableBodyCell>
        <TableBodyCell class="text-xl text-green-600 dark:text-green-300">
            +{data.new_submission_count}
        </TableBodyCell>
    </TableBodyRow>
{/if}
