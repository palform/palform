<script lang="ts">
    import {
        APIQuestionFileUploadType,
        type APIQuestionConfigurationOneOf6,
    } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import { Label, MultiSelect } from "flowbite-svelte";
    import {
        getFormEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/formEditor";

    export let config: APIQuestionConfigurationOneOf6;
    const ctx = getFormEditorCtx();
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
