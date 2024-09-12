<script lang="ts">
    import type { APIQuestionConfigurationOneOf7 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import { Alert, Toggle } from "flowbite-svelte";
    import { getFormEditorCtx, type QuestionEditEvents } from "../../../data/contexts/formEditor";

    export let config: APIQuestionConfigurationOneOf7;
    const ctx = getFormEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };
</script>

{#if !config.signature.allow_freeform && !config.signature.allow_initial && !config.signature.allow_full_name}
    <Alert class="mb-4">Please select at least one signature method</Alert>
{/if}

<Toggle
    bind:checked={config.signature.allow_freeform}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Allow freeform hand-drawn signatures
</Toggle>

<Toggle
    class="mt-4"
    bind:checked={config.signature.allow_initial}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Allow initials as signature
</Toggle>

<Toggle
    class="mt-4"
    bind:checked={config.signature.allow_full_name}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Allow full name as signature
</Toggle>
