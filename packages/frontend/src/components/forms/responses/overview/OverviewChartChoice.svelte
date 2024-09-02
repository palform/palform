<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/results";
    import { sGetChoice } from "../../../../data/contexts/fill";
    import { qIsChoice } from "../../../../data/contexts/questionsEditing";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    let series: number[] = [];
    $: {
        if ($question !== undefined && qIsChoice($question.configuration)) {
            series = $question?.configuration.choice.options.map((opt) =>
                $submissions.reduce(
                    (t, s) =>
                        t +
                        sGetChoice(s.data).option.filter((sOpt) => sOpt === opt)
                            .length,
                    0,
                ),
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
            labels: $question.configuration.choice.options,
        }}
    />
{/if}
