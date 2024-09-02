<script lang="ts">
    import type { APIQuestionConfigurationOneOf8 } from "@paltiverse/palform-typescript-openapi";
    import {
        getEditorCtx,
        type QuestionEditEvents,
    } from "../../../data/contexts/questionsEditing";
    import { createEventDispatcher } from "svelte";
    import { Button, ButtonGroup, Input, Toggle } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";

    export let config: APIQuestionConfigurationOneOf8;
    const ctx = getEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };

    $: onAddDim = (dim: "col" | "row") => {
        if (dim === "col") {
            config.choice_matrix.columns = [
                ...config.choice_matrix.columns,
                "",
            ];
        } else {
            config.choice_matrix.rows = [...config.choice_matrix.rows, ""];
        }
        dispatch("update", config);
    };

    $: onDel = (dim: "col" | "row", val: string) => {
        if (dim === "col") {
            config.choice_matrix.columns = config.choice_matrix.columns.filter(
                (e) => e !== val
            );
        } else {
            config.choice_matrix.rows = config.choice_matrix.rows.filter(
                (e) => e !== val
            );
        }
        dispatch("update", config);
    };
</script>

<Toggle
    class="mb-4"
    bind:checked={config.choice_matrix.multi_cols}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Allow selecting multiple columns in each row
</Toggle>

<div class="grid grid-cols-2 gap-4">
    <div class="space-y-2">
        <InfoText>Rows</InfoText>
        {#each config.choice_matrix.rows as row, index}
            <ButtonGroup>
                <Input
                    bind:value={row}
                    on:input={onUpdate}
                    disabled={$ctx.loading}
                />
                {#if index !== 0}
                    <Button
                        color="light"
                        outline
                        on:click={() => onDel("row", row)}
                        disabled={$ctx.loading}
                    >
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                {/if}
            </ButtonGroup>
        {/each}

        <Button
            class="block"
            color="light"
            size="sm"
            on:click={() => onAddDim("row")}
            disabled={$ctx.loading}
        >
            <FontAwesomeIcon icon={faPlus} />
            Add row
        </Button>
    </div>
    <div class="space-y-2">
        <InfoText>Columns</InfoText>
        {#each config.choice_matrix.columns as column, index}
            <ButtonGroup>
                <Input
                    bind:value={column}
                    on:input={onUpdate}
                    disabled={$ctx.loading}
                />
                {#if index !== 0}
                    <Button
                        color="light"
                        outline
                        on:click={() => onDel("col", column)}
                        disabled={$ctx.loading}
                    >
                        <FontAwesomeIcon icon={faTrash} />
                    </Button>
                {/if}
            </ButtonGroup>
        {/each}

        <Button
            class="block"
            color="light"
            size="sm"
            on:click={() => onAddDim("col")}
            disabled={$ctx.loading}
        >
            <FontAwesomeIcon icon={faPlus} />
            Add column
        </Button>
    </div>
</div>
