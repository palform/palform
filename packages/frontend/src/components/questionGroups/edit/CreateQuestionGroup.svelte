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

    interface Props {
        beforeIndex: number;
        alertMode?: boolean;
        class?: string;
        oncreate?: () => void;
    }

    let {
        beforeIndex,
        alertMode = false,
        class: className,
        oncreate,
    }: Props = $props();

    const formEditorCtx = getFormEditorCtx();
    const formCtx = getFormCtx();

    let showModal = $state(false);

    function onAddClick() {
        insertQuestionGroup(formEditorCtx, beforeIndex, null, null);
        showModal = false;
    }
</script>

{#if $formCtx.one_question_per_page}
    <CreateQuestion
        {beforeIndex}
        groupId={undefined}
        {alertMode}
        class={className}
        {oncreate}
    />
{:else}
    <Button
        onclick={onAddClick}
        color={alertMode ? "primary" : "light"}
        size={alertMode ? "sm" : "xs"}
        disabled={$formEditorCtx.loading ||
            $formEditorCtx.currentlyEditing !== undefined}
        class={className}
    >
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Add section
    </Button>
{/if}
