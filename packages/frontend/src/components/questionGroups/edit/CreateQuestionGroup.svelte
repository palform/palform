<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import LoadingButton from "../../LoadingButton.svelte";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import {
        getResponsesContext,
        insertGroup,
    } from "../../../data/contexts/results";
    import { showSuccessToast } from "../../../data/toast";
    import { getEditorCtx } from "../../../data/contexts/questionsEditing";
    import CreateQuestion from "../../questions/edit/CreateQuestion.svelte";

    export let beforeIndex: number;
    export let alertMode = false;
    const orgCtx = getOrgContext();
    const respCtx = getResponsesContext();
    const editorCtx = getEditorCtx();
    const formCtx = getFormCtx();

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

{#if $formCtx.one_question_per_page}
    <CreateQuestion
        {beforeIndex}
        groupId={undefined}
        {alertMode}
        class={$$props.class}
        on:create
    />
{:else}
    <LoadingButton
        on:click={onAddClick}
        outline={!alertMode}
        color={alertMode ? "primary" : "light"}
        size={alertMode ? "sm" : "xs"}
        disabled={$editorCtx.loading ||
            $editorCtx.currentlyEditing !== undefined}
        buttonClass={$$props.class}
    >
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Add section
    </LoadingButton>
{/if}
