<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import { sGetChoice } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsChoice } from "../../../../data/contexts/formEditor";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    $: uniqueChoices = [
        ...new Set($submissions.flatMap((e) => sGetChoice(e.data).option)),
    ];

    let series: number[] = [];
    $: {
        if ($question !== undefined && qIsChoice($question.configuration)) {
            series = uniqueChoices.map((opt) =>
                $submissions.reduce(
                    (t, s) =>
                        t +
                        sGetChoice(s.data).option.filter((sOpt) => sOpt === opt)
                            .length,
                    0
                )
            );
        }
    }
</script>

{#if $question !== undefined && qIsChoice($question.configuration)}
    <Chart
        options={{
            series: $question.configuration.choice.multi
                ? [
                      {
                          data: series,
                      },
                  ]
                : series,
            chart: {
                type: $question.configuration.choice.multi ? "bar" : "pie",
                height: 300,
            },
            labels: uniqueChoices,
        }}
    />
{/if}
