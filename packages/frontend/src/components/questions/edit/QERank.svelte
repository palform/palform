<script lang="ts">
    import type { ConfigRank } from "@palform/palform-typescript-openapi";
    import { getFormEditorCtx } from "../../../data/contexts/formEditor";
    import { Alert, Button, Toggle } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import {
        DragDropProvider,
        type DragDropEventHandlers,
    } from "@dnd-kit/svelte";
    import { isSortable } from "@dnd-kit/svelte/sortable";
    import type { DragDropManager, Draggable, Droppable } from "@dnd-kit/dom";
    import { RestrictToVerticalAxis } from "@dnd-kit/abstract/modifiers";
    import QERankOption from "./QERankOption.svelte";
    import StaticIndexMap from "../../../data/util/staticIndexMap";

    interface Props {
        config: ConfigRank;
    }

    let { config = $bindable() }: Props = $props();
    let ctx = getFormEditorCtx();

    let indexMap = $state(new StaticIndexMap(config.rank.options));

    let onOptionRemove = $derived((index: number) => {
        const newOptions = [...config.rank.options];
        newOptions.splice(index, 1);
        config.rank.options = newOptions;
        indexMap.delete(index);
    });
    let onOptionAdd = $derived(() => {
        config.rank.options = [...config.rank.options, ""];
        indexMap.insert();
    });

    let snapshot = $state<string[]>([]);
    let onDragStart = $derived(() => (snapshot = [...config.rank.options]));
    let onDragEnd = $derived((ev: { canceled: boolean }) => {
        if (ev.canceled) {
            config.rank.options = snapshot;
        }
    });

    let onDragOver: DragDropEventHandlers<
        {},
        Draggable,
        Droppable,
        DragDropManager
    >["onDragOver"] = $derived((event) => {
        const { source, target } = event.operation;
        if (!isSortable(source) || !isSortable(target)) return;

        const fromIndex = source.index;
        const toIndex = target.index;
        if (fromIndex !== toIndex) {
            const newItems = [...config.rank.options];
            const [removed] = newItems.splice(fromIndex, 1);
            newItems.splice(toIndex, 0, removed);
            config.rank.options = newItems;
            indexMap.move(fromIndex, toIndex);
        }
    });

    let isUnique = $derived(
        config.rank.options.every(
            (o: string, oi: number) =>
                !config.rank.options.some(
                    (e: string, ei: number) => o === e && oi !== ei
                )
        )
    );

    const dndModifiers = [RestrictToVerticalAxis];
    const id = $props.id();
</script>

{#if !isUnique}
    <Alert color="red" border class="mb-2">Options must be unique!</Alert>
{/if}

<Toggle
    class="mb-4"
    bind:checked={config.rank.default_random}
    disabled={$ctx.loading}
>
    Randomise initial order
</Toggle>

<DragDropProvider
    {onDragStart}
    {onDragEnd}
    {onDragOver}
    modifiers={dndModifiers}
>
    <div class="space-y-2">
        {#each config.rank.options as option, index (indexMap.getKey(index))}
            <QERankOption
                {option}
                {index}
                questionId={id}
                onremove={() => onOptionRemove(index)}
                bind:value={config.rank.options[index]}
            />
        {/each}
    </div>
</DragDropProvider>

<Button size="sm" class="mt-3" onclick={onOptionAdd} disabled={$ctx.loading}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add option
</Button>
