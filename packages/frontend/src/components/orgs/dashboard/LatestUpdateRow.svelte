<script lang="ts">
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import type { APISubmissionCountPerForm } from "@palform/palform-typescript-openapi";
    import { p } from "../../../router";
    interface Props {
        data: APISubmissionCountPerForm;
    }

    let { data }: Props = $props();
    const formCtx = $derived(getFormCtx(data.form_id));
    const orgCtx = getOrgContext();

    let team = $derived(
        $orgCtx.myTeams.find((e) => e.team_id === data.team_id)
    );
</script>

{#if $formCtx && team}
    <TableBodyRow>
        <TableBodyCell>
            <a
                href={p("/orgs/:orgId/forms/:formId/overview", {
                    params: { orgId: $orgCtx.org.id, formId: $formCtx.id },
                })}
            >
                <span class="block text-gray-500">
                    {team.name}
                </span>
                <span class="text-gray-800 dark:text-gray-300">
                    {$formCtx.editor_name}
                </span>
            </a>
        </TableBodyCell>
        <TableBodyCell class="text-xl text-green-600 dark:text-green-300">
            +{data.new_submission_count}
        </TableBodyCell>
    </TableBodyRow>
{/if}
