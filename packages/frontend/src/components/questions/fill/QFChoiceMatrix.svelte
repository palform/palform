<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import type { APIQuestionConfigurationOneOf8 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import {
        setQuestionValue,
        sGetChoiceMatrix,
    } from "../../../data/contexts/fill";
    import InfoText from "../../type/InfoText.svelte";
    import { Checkbox, Radio } from "flowbite-svelte";
    import {
        getBrandCtx,
        getPaddingAmountForBrand,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { getFromDodgyMap } from "../../../data/util/map";

    const dispatch = createEventDispatcher<{ change: undefined }>();
    const brandCtx = getBrandCtx();

    export let id: string;
    export let config: APIQuestionConfigurationOneOf8;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue
        ? sGetChoiceMatrix(currentValue)
        : { options: new Map<string, string[]>() };

    $: onToggle = (row: string, col: string) => {
        const currentRow = getFromDodgyMap(value.options, row);
        if (config.choice_matrix.multi_cols) {
            if (currentRow !== undefined) {
                if (currentRow.includes(col)) {
                    setQuestionValue(id, {
                        ChoiceMatrix: {
                            options: {
                                ...value.options,
                                [row]: currentRow.filter((e) => e !== col),
                            },
                        },
                    });
                } else {
                    setQuestionValue(id, {
                        ChoiceMatrix: {
                            options: {
                                ...value.options,
                                [row]: [...currentRow, col],
                            },
                        },
                    });
                }
            } else {
                setQuestionValue(id, {
                    ChoiceMatrix: {
                        options: {
                            ...value.options,
                            [row]: [col],
                        },
                    },
                });
            }
        } else {
            setQuestionValue(id, {
                ChoiceMatrix: {
                    options: {
                        ...value.options,
                        [row]: currentRow
                            ? currentRow.includes(col)
                                ? []
                                : [col]
                            : [col],
                    },
                },
            });
        }

        dispatch("change");
    };

    $: component = config.choice_matrix.multi_cols ? Checkbox : Radio;
    $: gridColumns = `repeat(${config.choice_matrix.columns.length + 1}, minmax(0, 1fr))`;
</script>

<div
    class="grid gap-2"
    style:grid-template-columns={gridColumns}
    style:padding-left={getPaddingAmountForBrand($brandCtx)}
    style:padding-right={getPaddingAmountForBrand($brandCtx)}
>
    {#each config.choice_matrix.columns as column, index}
        <InfoText
            class={`text-sm text-center ${index === 0 ? "col-start-2" : ""}`}
        >
            {column}
        </InfoText>
    {/each}
</div>

<div
    class="grid auto-rows-fr mt-4 overflow-y-hidden overflow-x-auto"
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
>
    {#each config.choice_matrix.rows as row}
        <div
            class="grid items-center bg-gray-50 odd:bg-gray-100 dark:bg-slate-800 dark:odd:bg-slate-800/50 py-2"
            style:grid-template-columns={gridColumns}
            style:padding-left={getPaddingAmountForBrand($brandCtx)}
            style:padding-right={getPaddingAmountForBrand($brandCtx)}
            style:padding-top={getPaddingAmountForBrand($brandCtx, true)}
            style:padding-bottom={getPaddingAmountForBrand($brandCtx, true)}
        >
            <InfoText class="text-sm">
                {row}
            </InfoText>
            {#each config.choice_matrix.columns as column}
                <svelte:component
                    this={component}
                    class="justify-center"
                    checked={getFromDodgyMap(value.options, row)?.includes(
                        column
                    )}
                    on:change={() => onToggle(row, column)}
                    value={column}
                    name={`${id}-${row}`}
                />
            {/each}
        </div>
    {/each}
</div>
