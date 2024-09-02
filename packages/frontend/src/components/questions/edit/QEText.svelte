<script lang="ts">
    import {
        APIQuestionTextValidator,
        type APIQuestionConfigurationOneOf1,
    } from "@paltiverse/palform-typescript-openapi";
    import { Label, Select, Toggle } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import {
        getEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/questionsEditing";

    export let config: APIQuestionConfigurationOneOf1;
    const ctx = getEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };

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

<Toggle
    bind:checked={config.text.is_long}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Long answer
</Toggle>

<Label class="mt-4">
    Validation
    <Select
        class="mt-1"
        bind:value={config.text.validator}
        items={validators}
        on:change={onUpdate}
        disabled={$ctx.loading}
    />
</Label>
