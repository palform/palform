<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Modal } from "flowbite-svelte";
    import { showFailureToast } from "../../../data/toast";
    import NewQuestionType from "./NewQuestionType.svelte";
    import { createEventDispatcher } from "svelte";
    import {
        getFormEditorCtx,
        insertQuestion,
        insertQuestionGroup,
    } from "../../../data/contexts/formEditor";

    // If groupId is undefined, this refers to group indexes. If groupId is defined, this refers to question indexes.
    export let beforeIndex: number;
    export let alertMode = false;
    // If undefined, a new group will be created for the question
    export let groupId: string | undefined;

    const formEditorCtx = getFormEditorCtx();
    let showTypeSelectDropdown = false;

    const dispatch = createEventDispatcher<{ create: undefined }>();

    $: onAddTypeClick = async (type: string) => {
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
            dispatch("create");
        } catch (e) {
            await showFailureToast(e);
        }
        $formEditorCtx.loading = false;
    };
</script>

<Button
    color={alertMode ? "primary" : "light"}
    size={alertMode ? "sm" : "xs"}
    outline={!alertMode}
    disabled={$formEditorCtx.loading ||
        $formEditorCtx.currentlyEditing !== undefined}
    on:click={() => (showTypeSelectDropdown = true)}
    class={$$props.class}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add question
</Button>

<Modal bind:open={showTypeSelectDropdown} outsideclose title="Add new question">
    <div class="space-y-2">
        <NewQuestionType
            title="Info"
            description="A title and description with no input"
            on:click={() => onAddTypeClick("info")}
        />
        <NewQuestionType
            title="Text"
            description="Simple text input with optional validation"
            on:click={() => onAddTypeClick("text")}
        />
        <NewQuestionType
            title="Choice"
            description="Single- or multi-select options"
            on:click={() => onAddTypeClick("choice")}
        />
        <NewQuestionType
            title="Choice matrix"
            description="Grid-like options with rows and columns"
            on:click={() => onAddTypeClick("choice_matrix")}
        />
        <NewQuestionType
            title="Scale"
            description="Numerical scale between any two integers"
            on:click={() => onAddTypeClick("scale")}
        />
        <NewQuestionType
            title="Date"
            description="Interactive date and/or time selection"
            on:click={() => onAddTypeClick("date_time")}
        />
        <NewQuestionType
            title="Address"
            description="Validated international postal address (with autocomplete)"
            on:click={() => onAddTypeClick("address")}
        />
        <NewQuestionType
            title="Phone number"
            description="Calling code and phone number pairing"
            on:click={() => onAddTypeClick("phone_number")}
        />
        <NewQuestionType
            title="File upload"
            description="Encrypt and upload any type of file"
            on:click={() => onAddTypeClick("file_upload")}
        />
        <NewQuestionType
            title="Signature"
            description="Electronic signature with support for different formats"
            on:click={() => onAddTypeClick("signature")}
        />
        <NewQuestionType
            title="Hidden"
            description="Import a value from a query parameter into the response"
            on:click={() => onAddTypeClick("hidden")}
        />
    </div>
</Modal>
