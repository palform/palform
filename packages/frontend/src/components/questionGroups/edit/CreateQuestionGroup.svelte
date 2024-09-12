<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { getFormCtx } from "../../../data/contexts/orgLayout";
    import CreateQuestion from "../../questions/edit/CreateQuestion.svelte";
    import {
        getFormEditorCtx,
        insertQuestionGroup,
    } from "../../../data/contexts/formEditor";
    import { Button } from "flowbite-svelte";

    export let beforeIndex: number;
    export let alertMode = false;
    const formEditorCtx = getFormEditorCtx();
    const formCtx = getFormCtx();

    let showModal = false;

    $: onAddClick = async () => {
        insertQuestionGroup(formEditorCtx, beforeIndex, null, null);
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
    <Button
        on:click={onAddClick}
        outline={!alertMode}
        color={alertMode ? "primary" : "light"}
        size={alertMode ? "sm" : "xs"}
        disabled={$formEditorCtx.loading ||
            $formEditorCtx.currentlyEditing !== undefined}
        class={$$props.class}
    >
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Add section
    </Button>
{/if}
