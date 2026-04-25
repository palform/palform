<script lang="ts">
    import type { ConfigChoiceMatrix } from "@palform/palform-typescript-openapi";
    import { Button, ButtonGroup, Input, Toggle } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";

    interface Props {
        config: ConfigChoiceMatrix;
    }

    let { config = $bindable() }: Props = $props();
    let ctx = getFormEditorCtx();

    function onAddDim(dim: "col" | "row") {
        if (dim === "col") {
            config.choice_matrix.columns = [
                ...config.choice_matrix.columns,
                "",
            ];
        } else {
            config.choice_matrix.rows = [...config.choice_matrix.rows, ""];
        }
    }

    function onDel(dim: "col" | "row", val: string) {
        if (dim === "col") {
            config.choice_matrix.columns = config.choice_matrix.columns.filter(
                (e: string) => e !== val
            );
        } else {
            config.choice_matrix.rows = config.choice_matrix.rows.filter(
                (e: string) => e !== val
            );
        }
    }
</script>

<Toggle
    class="mb-4"
    bind:checked={config.choice_matrix.multi_cols}
    disabled={$ctx.loading}
>
    Allow selecting multiple columns in each row
</Toggle>

<div class="grid grid-cols-2 gap-4">
    <div class="space-y-2">
        <InfoText>Rows</InfoText>
        {#each config.choice_matrix.rows as _, index}
            <ButtonGroup>
                <Input
                    bind:value={config.choice_matrix.rows[index]}
                    disabled={$ctx.loading}
                />
                {#if index !== 0}
                    <Button
                        color="light"
                        onclick={() =>
                            onDel("row", config.choice_matrix.rows[index])}
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
            onclick={() => onAddDim("row")}
            disabled={$ctx.loading}
        >
            <FontAwesomeIcon icon={faPlus} />
            Add row
        </Button>
    </div>
    <div class="space-y-2">
        <InfoText>Columns</InfoText>
        {#each config.choice_matrix.columns as _, index}
            <ButtonGroup>
                <Input
                    bind:value={config.choice_matrix.columns[index]}
                    disabled={$ctx.loading}
                />
                {#if index !== 0}
                    <Button
                        color="light"
                        onclick={() =>
                            onDel("col", config.choice_matrix.columns[index])}
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
            onclick={() => onAddDim("col")}
            disabled={$ctx.loading}
        >
            <FontAwesomeIcon icon={faPlus} />
            Add column
        </Button>
    </div>
</div>
