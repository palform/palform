<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Modal } from "flowbite-svelte";
    import { showFailureToast } from "../../../data/toast";
    import NewQuestionType from "./NewQuestionType.svelte";
    import {
        getFormEditorCtx,
        insertQuestion,
        insertQuestionGroup,
    } from "../../../data/contexts/formEditor";

    interface Props {
        // If groupId is undefined, this refers to group indexes. If groupId is defined, this refers to question indexes.
        beforeIndex: number;
        alertMode?: boolean;
        // If undefined, a new group will be created for the question
        groupId: string | undefined;
        class?: string;
        oncreate?: () => void;
    }

    let {
        beforeIndex,
        alertMode = false,
        groupId,
        class: className,
        oncreate,
    }: Props = $props();

    const formEditorCtx = getFormEditorCtx();
    let showTypeSelectDropdown = $state(false);

    async function onAddTypeClick(type: string) {
        $formEditorCtx.loading = true;

        try {
            let finalGroupId: string;
            if (groupId === undefined) {
                finalGroupId = insertQuestionGroup(
                    formEditorCtx,
                    beforeIndex,
                    null,
                    null
                );
            } else {
                finalGroupId = groupId;
            }

            const newId = insertQuestion(
                formEditorCtx,
                type,
                groupId === undefined ? 0 : beforeIndex,
                finalGroupId
            );

            $formEditorCtx.currentlyEditing = newId;
            showTypeSelectDropdown = false;
            oncreate?.();
        } catch (e) {
            await showFailureToast(e);
        }
        $formEditorCtx.loading = false;
    }
</script>

<Button
    color={alertMode ? "primary" : "light"}
    size={alertMode ? "sm" : "xs"}
    disabled={$formEditorCtx.loading ||
        $formEditorCtx.currentlyEditing !== undefined}
    onclick={() => (showTypeSelectDropdown = true)}
    class={className}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add question
</Button>

<Modal bind:open={showTypeSelectDropdown} outsideclose title="Add new question">
    <div class="space-y-2">
        <NewQuestionType
            title="Info"
            description="A title and description with no input"
            onclick={() => onAddTypeClick("info")}
        />
        <NewQuestionType
            title="Text"
            description="Simple text input with optional validation"
            onclick={() => onAddTypeClick("text")}
        />
        <NewQuestionType
            title="Choice"
            description="Single- or multi-select options"
            onclick={() => onAddTypeClick("choice")}
        />
        <NewQuestionType
            title="Choice matrix"
            description="Grid-like options with rows and columns"
            onclick={() => onAddTypeClick("choice_matrix")}
        />
        <NewQuestionType
            title="Rank"
            description="Reorderable ranked list of options"
            onclick={() => onAddTypeClick("rank")}
        />
        <NewQuestionType
            title="Scale"
            description="Numerical scale between any two integers"
            onclick={() => onAddTypeClick("scale")}
        />
        <NewQuestionType
            title="Date"
            description="Interactive date and/or time selection"
            onclick={() => onAddTypeClick("date_time")}
        />
        <NewQuestionType
            title="Address"
            description="Validated international postal address (with autocomplete)"
            onclick={() => onAddTypeClick("address")}
        />
        <NewQuestionType
            title="Phone number"
            description="Calling code and phone number pairing"
            onclick={() => onAddTypeClick("phone_number")}
        />
        <NewQuestionType
            title="File upload"
            description="Encrypt and upload any type of file"
            onclick={() => onAddTypeClick("file_upload")}
        />
        <NewQuestionType
            title="Signature"
            description="Electronic signature with support for different formats"
            onclick={() => onAddTypeClick("signature")}
        />
        <NewQuestionType
            title="Hidden"
            description="Import a value from a query parameter into the response"
            onclick={() => onAddTypeClick("hidden")}
        />
    </div>
</Modal>
