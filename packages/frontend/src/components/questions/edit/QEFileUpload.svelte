<script lang="ts">
    import {
        APIQuestionFileUploadType,
        type APIQuestionConfigurationOneOf6,
    } from "@paltiverse/palform-typescript-openapi";
    import {
        getEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/questionsEditing";
    import { createEventDispatcher } from "svelte";
    import { Label, MultiSelect } from "flowbite-svelte";

    export let config: APIQuestionConfigurationOneOf6;
    const ctx = getEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    const possibleTypes = Object.values(APIQuestionFileUploadType).map((e) => ({
        name: e,
        value: e,
    }));

    $: onUpdate = () => {
        dispatch("update", config);
    };
</script>

<Label>
    Accepted file types
    <MultiSelect
        items={possibleTypes}
        bind:value={config.file_upload.allowed_types}
        class="mt-1"
        on:change={onUpdate}
        disabled={$ctx.loading}
    />
</Label>
