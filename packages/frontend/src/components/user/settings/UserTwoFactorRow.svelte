<script lang="ts">
    import { faKey, faMobile } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIAdminUserSecondAuthenticationFactor } from "@palform/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    interface Props {
        method: APIAdminUserSecondAuthenticationFactor;
        ondelete: () => void;
    }

    let { method, ondelete }: Props = $props();

    let loading = $state(false);
    let onDeleteClick = $derived(async () => {
        loading = true;
        try {
            await APIs.secondFactors().then((a) =>
                a.userSecondFactorsDelete(method.id)
            );
            await showSuccessToast("Method deleted");
            ondelete();
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        {#if method.method === "Webauthn"}
            <FontAwesomeIcon icon={faKey} class="me-1" />
        {:else if method.method === "TOTP"}
            <FontAwesomeIcon icon={faMobile} class="me-1" />
        {/if}
        {method.nickname}
    </TableBodyCell>
    <TableBodyCell>
        Created {parseServerTime(method.created_at).toRelative()}
    </TableBodyCell>
    <TableBodyCell>
        <TableSingleAction onclick={onDeleteClick} disabled={loading}>
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
