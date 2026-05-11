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
</script>

<Toggle class="mb-4" bind:checked={config.choice.multi} disabled={$ctx.loading}>
    Multi select
</Toggle>

{#if !isUnique}
    <Alert color="red" border class="mb-2">Options must be unique!</Alert>
{/if}

<div class="space-y-2">
    {#each config.choice.options as _, index}
        <div class="flex gap-x-2">
            <ButtonInput
                bind:value={config.choice.options[index]}
                disabled={$ctx.loading}
                hideButton={index === 0}
                onclick={() => onOptionRemove(config.choice.options[index])}
            >
                <FontAwesomeIcon icon={faTrash} />
            </ButtonInput>
        </div>
    {/each}
</div>

<Button size="sm" class="mt-3" onclick={onOptionAdd} disabled={$ctx.loading}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add option
</Button>
