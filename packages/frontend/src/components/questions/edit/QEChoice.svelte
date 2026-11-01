<script lang="ts">
    import type { ConfigChoice } from "@palform/palform-typescript-openapi";
    import { Alert, Button, Input, Toggle } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";
    import ButtonInput from "../../input/ButtonInput.svelte";

    interface Props {
        config: ConfigChoice;
    }

    let { config = $bindable() }: Props = $props();
    let ctx = getFormEditorCtx();

    function onOptionAdd() {
        config.choice.options = [...config.choice.options, ""];
    }
    function onOptionRemove(value: string) {
        config.choice.options = config.choice.options.filter(
            (e: string) => e !== value
        );
    }
    let isUnique = $derived(
        config.choice.options.every(
            (o: string, oi: number) =>
                !config.choice.options.some(
                    (e: string, ei: number) => o === e && oi !== ei
                )
        )
    );
    let containsForbiddenLabel = $derived(
        config.choice.options.some((e) => e === "__other__")
    );
</script>

<Toggle class="mb-4" bind:checked={config.choice.multi} disabled={$ctx.loading}>
    Multi select
</Toggle>

<Toggle
    class="mb-4"
    bind:checked={config.choice.show_other}
    disabled={$ctx.loading}
>
    Show "other" option
</Toggle>

{#if !isUnique}
    <Alert color="red" border class="mb-2">Options must be unique!</Alert>
{/if}

{#if containsForbiddenLabel}
    <Alert color="red" border class="mb-2"
        >__other__ is a reserved value and is not allowed. Please remove it.</Alert
    >
{/if}

<div class="space-y-2">
    {#each config.choice.options as _, index}
        <ButtonInput
            bind:value={config.choice.options[index]}
            disabled={$ctx.loading}
            hideButton={index === 0}
            onclick={() => onOptionRemove(config.choice.options[index])}
        >
            <FontAwesomeIcon icon={faTrash} />
        </ButtonInput>
    {/each}
    {#if config.choice.show_other}
        <div class="flex gap-x-2">
            <Input value="Other" disabled />
        </div>
    {/if}
</div>

<Button size="sm" class="mt-3" onclick={onOptionAdd} disabled={$ctx.loading}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add option
</Button>
