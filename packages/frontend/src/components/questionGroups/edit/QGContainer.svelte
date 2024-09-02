<script lang="ts">
    import { faEdit, faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIQuestionGroup } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import {
        ctxUpdateGroup,
        getGroupTitle,
        getResponsesContext,
    } from "../../../data/contexts/results";
    import {
        Button,
        ButtonGroup,
        Input,
        Label,
        Textarea,
        Tooltip,
    } from "flowbite-svelte";
    import { APIs } from "../../../data/common";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { getEditorCtx } from "../../../data/contexts/questionsEditing";

    export let group: APIQuestionGroup;
    export let loading: boolean;
    const dispatch = createEventDispatcher<{
        delete: undefined;
    }>();
    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    const respCtx = getResponsesContext();
    const editorCtx = getEditorCtx();

    let groupTitle = group.title;
    let groupDescription = group.description;

    let editing = false;
    $: onSaveClick = async () => {
        loading = true;
        try {
            const updatedGroup = {
                ...group,
                title: groupTitle,
                description: groupDescription,
            };
            await APIs.questionGroups().then((a) =>
                a.questionGroupsSet(
                    $orgCtx.org.id,
                    $formCtx.id,
                    group.id,
                    updatedGroup
                )
            );
            ctxUpdateGroup(respCtx, group.id, updatedGroup);
            await showSuccessToast("Group details saved");
            editing = false;
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
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
                                <Input
                                    bind:value={groupTitle}
                                    disabled={loading}
                                />
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
                    <Button
                        size="sm"
                        on:click={() => (groupTitle = "")}
                        disabled={loading}
                    >
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
                                <Textarea
                                    bind:value={groupDescription}
                                    disabled={loading}
                                />
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
                    <Button
                        size="sm"
                        on:click={() => (groupDescription = "")}
                        disabled={loading}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add description
                    </Button>
                {/if}

                {#if changed}
                    <LoadingButton
                        buttonClass="mt-4 block"
                        on:click={onSaveClick}
                        disabled={loading}
                        {loading}
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
            {:else}
                <h2 class="text-lg dark:text-gray-300">
                    {getGroupTitle(group)}
                </h2>
                {#if group.description}
                    <p class="text-gray-600 dark:text-gray-400">
                        {group.description}
                    </p>
                {/if}
            {/if}
        </div>
        <div>
            {#if !editing}
                <Button
                    color="light"
                    outline
                    size="sm"
                    disabled={loading || !!$editorCtx.currentlyEditing}
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
                {loading}
                disabled={loading || editing || !!$editorCtx.currentlyEditing}
            >
                <FontAwesomeIcon icon={faTrash} />
            </LoadingButton>
        </div>
    </div>

    {#if !editing}
        <slot />
    {/if}
</section>
