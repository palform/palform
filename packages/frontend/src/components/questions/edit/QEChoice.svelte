<script lang="ts">
    import type { APIQuestionConfigurationOneOf2 } from "@paltiverse/palform-typescript-openapi";
    import {
        getEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/questionsEditing";
    import { createEventDispatcher } from "svelte";
    import { Alert, Button, Input, Toggle } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";

    export let config: APIQuestionConfigurationOneOf2;
    const ctx = getEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };
    $: onOptionAdd = () => {
        config.choice.options = [...config.choice.options, ""];
    };
    $: onOptionRemove = (value: string) => {
        config.choice.options = config.choice.options.filter(
            (e) => e !== value,
        );
    };
    $: isUnique = config.choice.options.every(
        (o, oi) => !config.choice.options.some((e, ei) => o === e && oi !== ei),
    );
</script>

<Toggle
    class="mb-4"
    bind:checked={config.choice.multi}
    disabled={$ctx.loading}
    on:change={onUpdate}
>
    Multi select
</Toggle>

{#if !isUnique}
    <Alert color="red" border class="mb-2">Options must be unique!</Alert>
{/if}

<div class="space-y-2">
    {#each config.choice.options as option, index}
        <div class="flex gap-x-2">
            <Input
                size="sm"
                bind:value={option}
                on:change={onUpdate}
                disabled={$ctx.loading}
            />
            {#if index !== 0}
                <Button
                    disabled={$ctx.loading}
                    on:click={() => onOptionRemove(option)}
                >
                    <FontAwesomeIcon icon={faTrash} />
                </Button>
            {/if}
        </div>
    {/each}
</div>

<Button size="sm" class="mt-3" on:click={onOptionAdd} disabled={$ctx.loading}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add option
</Button>
