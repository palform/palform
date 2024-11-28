<script lang="ts">
    import { faKey, faMobile } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIAdminUserSecondAuthenticationFactor } from "@paltiverse/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { createEventDispatcher } from "svelte";

    export let method: APIAdminUserSecondAuthenticationFactor;
    const dispatch = createEventDispatcher<{ delete: undefined }>();

    let loading = false;
    $: onDeleteClick = async () => {
        loading = true;
        try {
            await APIs.secondFactors().then((a) =>
                a.userSecondFactorsDelete(method.id)
            );
            await showSuccessToast("Method deleted");
            dispatch("delete");
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
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
        <TableSingleAction on:click={onDeleteClick} disabled={loading}>
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
