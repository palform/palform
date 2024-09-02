<script lang="ts">
    import type { APIOrganisationAuthTeamMapping } from "@paltiverse/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow, Tooltip } from "flowbite-svelte";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import TableSingleAction from "../../../tables/TableSingleAction.svelte";
    import { APIs } from "../../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../../data/toast";
    import { createEventDispatcher } from "svelte";

    export let mapping: APIOrganisationAuthTeamMapping;
    const dispatch = createEventDispatcher<{ delete: undefined }>();
    const orgCtx = getOrgContext();

    let deleteLoading = false;
    $: onDeleteClick = async () => {
        deleteLoading = true;
        try {
            await APIs.orgAuthTeamMappings().then((a) =>
                a.organisationAuthConfigMappingsDelete(
                    $orgCtx.org.id,
                    mapping.id
                )
            );
            await showSuccessToast("Mapping deleted");
            dispatch("delete");
        } catch (e) {
            await showFailureToast(e);
        }
        deleteLoading = false;
    };
</script>

<TableBodyRow>
    <TableBodyCell>{mapping.field_value}</TableBodyCell>
    <TableBodyCell>
        {mapping.team_name}
    </TableBodyCell>
    <TableBodyCell>{mapping.role}</TableBodyCell>
    <TableBodyCell>
        <TableSingleAction disabled={deleteLoading} on:click={onDeleteClick}>
            Delete
        </TableSingleAction>
        <Tooltip>This won't affect existing team memberships.</Tooltip>
    </TableBodyCell>
</TableBodyRow>
