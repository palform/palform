<script lang="ts">
    import type { APIOrganisationAuthTeamMapping } from "@palform/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow, Tooltip } from "flowbite-svelte";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import TableSingleAction from "../../../tables/TableSingleAction.svelte";
    import { APIs } from "../../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../../data/toast";

    interface Props {
        mapping: APIOrganisationAuthTeamMapping;
        ondelete: () => void;
    }

    let { mapping, ondelete }: Props = $props();
    const orgCtx = getOrgContext();

    let deleteLoading = $state(false);
    let onDeleteClick = $derived(async () => {
        deleteLoading = true;
        try {
            await APIs.orgAuthTeamMappings().then((a) =>
                a.organisationAuthConfigMappingsDelete(
                    $orgCtx.org.id,
                    mapping.id
                )
            );
            await showSuccessToast("Mapping deleted");
            ondelete();
        } catch (e) {
            await showFailureToast(e);
        }
        deleteLoading = false;
    });
</script>

<TableBodyRow>
    <TableBodyCell>{mapping.field_value}</TableBodyCell>
    <TableBodyCell>
        {mapping.team_name}
    </TableBodyCell>
    <TableBodyCell>{mapping.role}</TableBodyCell>
    <TableBodyCell>
        <TableSingleAction disabled={deleteLoading} onclick={onDeleteClick}>
            Delete
        </TableSingleAction>
        <Tooltip>This won't affect existing team memberships.</Tooltip>
    </TableBodyCell>
</TableBodyRow>
