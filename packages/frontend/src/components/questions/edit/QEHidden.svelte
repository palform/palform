<script lang="ts">
    import type { APIQuestionConfigurationOneOf10 } from "@paltiverse/palform-typescript-openapi";
    import {
        getEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/questionsEditing";
    import { createEventDispatcher } from "svelte";
    import { Helper, Input, Label } from "flowbite-svelte";

    export let config: APIQuestionConfigurationOneOf10;
    const ctx = getEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };
</script>

<Label>
    Query parameter name
    <Input
        class="mt-2"
        bind:value={config.hidden.parameter_name}
        on:input={onUpdate}
        disabled={$ctx.loading}
    />
    {#if config.hidden.parameter_name.length > 0}
        <Helper class="mt-2">
            We'll take the value from the parameter <code
                >?{config.hidden.parameter_name}=</code
            > in the URL when the form is being filled out.
        </Helper>
    {/if}
</Label>
