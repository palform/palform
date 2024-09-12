<script lang="ts">
    import { faEdit, faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIQuestionGroup } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import {
        Button,
        ButtonGroup,
        Input,
        Label,
        Textarea,
        Tooltip,
    } from "flowbite-svelte";
    import { getFormCtx } from "../../../data/contexts/orgLayout";
    import { showFailureToast } from "../../../data/toast";
    import {
        getFormAdminContext,
        getGroupTitle,
    } from "../../../data/contexts/formAdmin";
    import {
        getFormEditorCtx,
        updateQuestionGroup,
    } from "../../../data/contexts/formEditor";

    export let group: APIQuestionGroup;

    const dispatch = createEventDispatcher<{
        delete: undefined;
    }>();

    const formMetadataCtx = getFormCtx();
    const formAdminCtx = getFormAdminContext();
    const formEditorCtx = getFormEditorCtx();

    let groupTitle = group.title;
    let groupDescription = group.description;

    let editing = false;
    $: onSaveClick = async () => {
        try {
            const updatedGroup = {
                ...group,
                title: groupTitle,
                description: groupDescription,
            };
            updateQuestionGroup(formEditorCtx, updatedGroup);
            editing = false;
        } catch (e) {
            await showFailureToast(e);
        }
    };
    $: changed =
        groupTitle !== group.title || groupDescription !== group.description;
</script>

<section
    class="border border-slate-300 dark:border-slate-600 shadow-sm rounded-xl p-4"
>
    <div class="mb-4 flex justify-between gap-8">
        <div class="flex-1">
            {#if editing}
                {#if typeof groupTitle === "string"}
                    <div class="flex justify-between gap-4">
                        <Label class="flex-1">
                            Title
                            <ButtonGroup class="flex mt-1">
                                <Input bind:value={groupTitle} />
                                <Button
                                    color="light"
                                    outline
                                    on:click={() => (groupTitle = null)}
                                >
                                    <FontAwesomeIcon icon={faTrash} />
                                </Button>
                                <Tooltip>Delete title</Tooltip>
                            </ButtonGroup>
                        </Label>
                    </div>
                {:else}
                    <Button size="sm" on:click={() => (groupTitle = "")}>
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add title
                    </Button>
                {/if}

                <div class="h-4" />

                {#if typeof groupDescription === "string"}
                    <div class="flex justify-between gap-4">
                        <Label class="flex-1">
                            Description
                            <ButtonGroup class="flex mt-1">
                                <Textarea bind:value={groupDescription} />
                                <Button
                                    color="light"
                                    outline
                                    on:click={() => (groupDescription = null)}
                                >
                                    <FontAwesomeIcon icon={faTrash} />
                                </Button>
                                <Tooltip>Delete description</Tooltip>
                            </ButtonGroup>
                        </Label>
                    </div>
                {:else}
                    <Button size="sm" on:click={() => (groupDescription = "")}>
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add description
                    </Button>
                {/if}

                {#if changed}
                    <LoadingButton
                        buttonClass="mt-4 block"
                        on:click={onSaveClick}
                    >
                        Save
                    </LoadingButton>
                {:else}
                    <Button
                        on:click={() => (editing = false)}
                        color="light"
                        size="sm"
                        class="mt-4 block"
                    >
                        Cancel
                    </Button>
                {/if}
            {:else if !$formMetadataCtx.one_question_per_page}
                <h2 class="text-lg dark:text-gray-300">
                    {getGroupTitle(false, $formAdminCtx, group)}
                </h2>
                {#if group.description}
                    <p class="text-gray-600 dark:text-gray-400">
                        {group.description}
                    </p>
                {/if}
            {/if}
        </div>
        {#if !$formMetadataCtx.one_question_per_page}
            <div>
                {#if !editing}
                    <Button
                        color="light"
                        outline
                        size="sm"
                        disabled={!!$formEditorCtx.currentlyEditing}
                        on:click={() => (editing = true)}
                    >
                        <FontAwesomeIcon icon={faEdit} />
                    </Button>
                {/if}

                <LoadingButton
                    size="sm"
                    outline
                    color="red"
                    title="Delete section"
                    on:click={() => dispatch("delete")}
                    disabled={editing || !!$formEditorCtx.currentlyEditing}
                >
                    <FontAwesomeIcon icon={faTrash} />
                </LoadingButton>
            </div>
        {/if}
    </div>

    {#if !editing}
        <slot />
    {/if}
</section>
