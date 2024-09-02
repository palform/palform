<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import LoadingButton from "../../LoadingButton.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import {
        getResponsesContext,
        insertGroup,
    } from "../../../data/contexts/results";
    import { showSuccessToast } from "../../../data/toast";
    import { getEditorCtx } from "../../../data/contexts/questionsEditing";

    export let beforeIndex: number;
    export let alertMode = false;
    const orgCtx = getOrgContext();
    const respCtx = getResponsesContext();
    const editorCtx = getEditorCtx();

    let showModal = false;
    let addLoading = false;

    $: onAddClick = async () => {
        addLoading = true;
        await insertGroup(respCtx, $orgCtx.org.id, beforeIndex, null, null);
        await showSuccessToast("Section created");
        addLoading = false;
        showModal = false;
    };
</script>

<LoadingButton
    on:click={onAddClick}
    outline={!alertMode}
    color={alertMode ? "primary" : "light"}
    size={alertMode ? "sm" : "xs"}
    disabled={$editorCtx.loading || $editorCtx.currentlyEditing !== undefined}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add section
</LoadingButton>
