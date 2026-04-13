<script lang="ts">
    import {
        APIQuestionFileUploadType,
        type ConfigFileUpload,
    } from "@palform/palform-typescript-openapi";
    import { Label, MultiSelect } from "flowbite-svelte";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        config: ConfigFileUpload;
    }

    let { config = $bindable() }: Props = $props();
    const ctx = getFormEditorCtx();

    const possibleTypes = Object.values(APIQuestionFileUploadType).map((e) => ({
        name: e,
        value: e,
    }));
</script>

<Label>
    Accepted file types
    <MultiSelect
        items={possibleTypes}
        bind:value={config.file_upload.allowed_types}
        class="mt-1"
        disabled={$ctx.loading}
    />
</Label>
