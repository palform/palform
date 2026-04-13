<script lang="ts">
    import {
        setQuestionValue,
        sGetChoiceMatrix,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import InfoText from "../../type/InfoText.svelte";
    import { Checkbox, Radio } from "flowbite-svelte";
    import {
        getBrandCtx,
        getPaddingAmountForBrand,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { getFromDodgyMap } from "../../../data/util/map";
    import type { ConfigChoiceMatrix } from "@palform/palform-typescript-openapi";

    const brandCtx = getBrandCtx();

    interface Props extends QuestionFillProps<ConfigChoiceMatrix> {}

    let { id, config, currentValue, onchange }: Props = $props();
    let value = $derived(
        currentValue
            ? sGetChoiceMatrix(currentValue)
            : { options: new Map<string, string[]>() }
    );

    let onToggle = $derived((row: string, col: string) => {
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

        onchange();
    });

    let component = $derived(
        config.choice_matrix.multi_cols ? Checkbox : Radio
    );
    let gridColumns = $derived(
        `repeat(${config.choice_matrix.columns.length + 1}, minmax(0, 1fr))`
    );
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
            class="grid items-center bg-gray-50 odd:bg-gray-100 dark:bg-slate-800 dark:odd:bg-slate-800/50"
            style:grid-template-columns={gridColumns}
            style:padding-left={getPaddingAmountForBrand($brandCtx)}
            style:padding-right={getPaddingAmountForBrand($brandCtx)}
        >
            <InfoText
                class="text-sm py-2"
                style={`padding-top: ${getPaddingAmountForBrand($brandCtx, true)}; padding-bottom: ${getPaddingAmountForBrand($brandCtx, true)};`}
            >
                {row}
            </InfoText>
            {#each config.choice_matrix.columns as column}
                {@const SvelteComponent = component}
                <label
                    class="py-2 flex items-center justify-center"
                    style:padding-top={getPaddingAmountForBrand(
                        $brandCtx,
                        true
                    )}
                    style:padding-bottom={getPaddingAmountForBrand(
                        $brandCtx,
                        true
                    )}
                >
                    <SvelteComponent
                        checked={getFromDodgyMap(value.options, row)?.includes(
                            column
                        )}
                        onchange={() => onToggle(row, column)}
                        value={column}
                        name={`${id}-${row}`}
                    />
                </label>
            {/each}
        </div>
    {/each}
</div>
