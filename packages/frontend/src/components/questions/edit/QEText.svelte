<script lang="ts">
    import {
        APIQuestionTextValidator,
        type ConfigText,
    } from "@palform/palform-typescript-openapi";
    import { Label, Select, Toggle } from "flowbite-svelte";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        config: ConfigText;
    }

    let { config = $bindable() }: Props = $props();
    const ctx = getFormEditorCtx();

    const validators = [
        {
            name: "None",
            value: null,
        },
        ...Object.values(APIQuestionTextValidator).map((v) => ({
            name: v,
            value: v,
        })),
    ] as { name: string; value: string | null }[];
</script>

<Toggle bind:checked={config.text.is_long} disabled={$ctx.loading}>
    Long answer
</Toggle>

<Label class="mt-4">
    Validation
    <Select
        class="mt-1"
        bind:value={config.text.validator}
        items={validators}
        disabled={$ctx.loading}
    />
</Label>
