<script lang="ts">
    import { sGetChoiceMatrix } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsChoiceMatrix } from "../../../../data/contexts/formEditor";
    import { Chart } from "@flowbite-svelte-plugins/chart";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let question = $derived(ctxGetQuestion(questionId));
    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    const series: () => ApexAxisChartSeries = $derived(() => {
        if ($question && qIsChoiceMatrix($question.configuration)) {
            const _series: ApexAxisChartSeries = [];

            for (const col of $question.configuration.choice_matrix.columns) {
                const seriesData: number[] = [];

                for (const row of $question.configuration.choice_matrix.rows) {
                    const count = $submissions.reduce((t, c) => {
                        const o = sGetChoiceMatrix(c.data).options;

                        if (o.get(row)?.includes(col)) {
                            return t + 1;
                        }

                        return t;
                    }, 0);

                    seriesData.push(count);
                }

                _series.push({
                    name: col,
                    data: seriesData,
                });
            }

            return _series;
        }

        return [];
    });
</script>

{#if $question !== undefined && qIsChoiceMatrix($question.configuration)}
    <Chart
        options={{
            series: series(),
            chart: {
                type: "bar",
                height: 350,
                stacked: true,
                stackType: $question.configuration.choice_matrix.multi_cols
                    ? "normal"
                    : "100%",
            },
            labels: $question.configuration.choice_matrix.rows,
            legend: {
                position: "right",
            },
        }}
    />
{/if}
