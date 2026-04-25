<script lang="ts">
    import {
        faArrowDown,
        faArrowUp,
        faEdit,
        faPlus,
        faTrash,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIQuestionGroup } from "@palform/palform-typescript-openapi";
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
        moveQuestionGroup,
        updateQuestionGroup,
    } from "../../../data/contexts/formEditor";
    import type { ArrayMoveDirection } from "../../../data/util/arraySwap";

    interface Props {
        group: APIQuestionGroup;
        children?: import("svelte").Snippet;
        ondelete: () => void;
    }

    let { group, children, ondelete }: Props = $props();

    const formMetadataCtx = getFormCtx();
    const formAdminCtx = getFormAdminContext();
    const formEditorCtx = getFormEditorCtx();

    // svelte-ignore state_referenced_locally
    let groupTitle = $state(group.title);
    // svelte-ignore state_referenced_locally
    let groupDescription = $state(group.description);

    let editing = $state(false);
    let onSaveClick = $derived(async () => {
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
    });
    let changed = $derived(
        groupTitle !== group.title || groupDescription !== group.description
    );

    let groupIndex = $derived(
        $formEditorCtx.groups.findIndex((e) => e.id === group.id)
    );
    let canMoveUp = $derived(groupIndex > 0);
    let canMoveDown = $derived(groupIndex !== $formEditorCtx.groups.length - 1);
    let onMoveClick = $derived((direction: ArrayMoveDirection) => {
        moveQuestionGroup(formEditorCtx, group, direction);
    });
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
                                    onclick={() => (groupTitle = null)}
                                    aria-label="Delete title"
                                >
                                    <FontAwesomeIcon icon={faTrash} />
                                </Button>
                            </ButtonGroup>
                        </Label>
                    </div>
                {:else}
                    <Button size="sm" onclick={() => (groupTitle = "")}>
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add title
                    </Button>
                {/if}

                <div class="h-4"></div>

                {#if typeof groupDescription === "string"}
                    <div class="flex justify-between gap-4">
                        <Label class="flex-1">
                            Description
                            <ButtonGroup class="flex mt-1">
                                <Textarea
                                    bind:value={groupDescription}
                                    class="w-full"
                                />
                                <Button
                                    color="light"
                                    onclick={() => (groupDescription = null)}
                                    aria-label="Delete description"
                                >
                                    <FontAwesomeIcon icon={faTrash} />
                                </Button>
                            </ButtonGroup>
                        </Label>
                    </div>
                {:else}
                    <Button size="sm" onclick={() => (groupDescription = "")}>
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add description
                    </Button>
                {/if}

                {#if changed}
                    <LoadingButton
                        buttonClass="mt-4 block"
                        onclick={onSaveClick}
                    >
                        Save
                    </LoadingButton>
                {:else}
                    <Button
                        onclick={() => (editing = false)}
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
        <div>
            <ButtonGroup>
                <Button
                    onclick={() => onMoveClick("up")}
                    disabled={!canMoveUp}
                    color="light"
                >
                    <FontAwesomeIcon icon={faArrowUp} />
                </Button>
                <Button
                    onclick={() => onMoveClick("down")}
                    disabled={!canMoveDown}
                    color="light"
                >
                    <FontAwesomeIcon icon={faArrowDown} />
                </Button>
            </ButtonGroup>

            {#if !$formMetadataCtx.one_question_per_page}
                {#if !editing}
                    <Button
                        color="light"
                        size="sm"
                        disabled={!!$formEditorCtx.currentlyEditing}
                        onclick={() => (editing = true)}
                    >
                        <FontAwesomeIcon icon={faEdit} />
                    </Button>
                {/if}

                <LoadingButton
                    size="sm"
                    outline
                    color="red"
                    title="Delete section"
                    onclick={() => ondelete()}
                    disabled={editing || !!$formEditorCtx.currentlyEditing}
                >
                    <FontAwesomeIcon icon={faTrash} />
                </LoadingButton>
            {/if}
        </div>
    </div>

    {#if !editing}
        {@render children?.()}
    {/if}
</section>
