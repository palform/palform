<script lang="ts">
    import type { ConfigRank } from "@palform/palform-typescript-openapi";
    import {
        fillSendStore,
        setQuestionValue,
        sGetRank,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import { isSortable } from "@dnd-kit/svelte/sortable";
    import {
        DragDropProvider,
        type DragDropEventHandlers,
    } from "@dnd-kit/svelte";
    import {
        Accessibility,
        type DragDropManager,
        type Draggable,
        type Droppable,
    } from "@dnd-kit/dom";
    import { RestrictToVerticalAxis } from "@dnd-kit/abstract/modifiers";
    import QFClearButton from "./QFClearButton.svelte";
    import { t } from "../../../data/contexts/i18n";
    import QFRankOption from "./QFRankOption.svelte";

    interface Props extends QuestionFillProps<ConfigRank> {}

    let { id, config, currentValue, onchange }: Props = $props();
    let value = $derived(currentValue ? sGetRank(currentValue) : { value: [] });

    let snapshot = $state<string[]>([]);
    let onDragStart = $derived(() => (snapshot = [...value.value]));
    let onDragEnd = $derived((ev: { canceled: boolean }) => {
        if (ev.canceled) {
            setQuestionValue(id, {
                Rank: {
                    value: snapshot,
                },
            });
            onchange();
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
            const newItems = [...value.value];
            const [removed] = newItems.splice(fromIndex, 1);
            newItems.splice(toIndex, 0, removed);
            setQuestionValue(id, {
                Rank: {
                    value: newItems,
                },
            });
            onchange();
        }
    });

    let hasChanged = $derived.by(() => {
        return value.value.some((v, i) => config.rank.options[i] !== v);
    });
    let onReset = $derived(() => {
        setQuestionValue(id, {
            Rank: {
                value: config.rank.options,
            },
        });
        onchange();
    });

    let onMove = $derived((index: number, direction: "up" | "down") => {
        const newItems = [...value.value];
        const [removed] = newItems.splice(index, 1);
        newItems.splice(direction === "up" ? index - 1 : index + 1, 0, removed);
        setQuestionValue(id, {
            Rank: {
                value: newItems,
            },
        });
        onchange();
    });

    const dndPlugins = [Accessibility.configure({})];
    const dndModifiers = [RestrictToVerticalAxis];
</script>

<DragDropProvider
    {onDragStart}
    {onDragEnd}
    {onDragOver}
    plugins={(defaults) => [...defaults, ...dndPlugins]}
    modifiers={dndModifiers}
>
    <ol class="space-y-2">
        {#each value.value as option, index (option)}
            <QFRankOption
                {option}
                {index}
                questionId={id}
                optionsCount={value.value.length}
                onMove={(direction) => onMove(index, direction)}
            />
        {/each}
    </ol>
</DragDropProvider>

<QFClearButton
    class="mt-2"
    onclick={onReset}
    disabled={$fillSendStore?.loading}
    text={t("field_reset")}
    hidden={config.rank.default_random || !hasChanged}
/>
