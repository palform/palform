<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import { sGetChoiceMatrix } from "../../../../data/contexts/fill";
    import { ctxGetQuestion, ctxSubmissionsForQuestion } from "../../../../data/contexts/formAdmin";
    import { qIsChoiceMatrix } from "../../../../data/contexts/formEditor";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    let series: ApexAxisChartSeries | null = null;
    $: {
        if ($question && qIsChoiceMatrix($question.configuration)) {
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

                series = [
                    ...(series ?? []),
                    {
                        name: col,
                        data: seriesData,
                    },
                ];
            }
        }
    }
</script>

{#if series && $question !== undefined && qIsChoiceMatrix($question.configuration)}
    <Chart
        options={{
            series,
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
