<script lang="ts">
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import { sGetChoice } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsChoice } from "../../../../data/contexts/formEditor";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let question = $derived(ctxGetQuestion(questionId));
    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    let uniqueChoices = $derived([
        ...new Set($submissions.flatMap((e) => sGetChoice(e.data).option)),
    ]);

    const series: () => number[] = $derived(() => {
        if ($question !== undefined && qIsChoice($question.configuration)) {
            return uniqueChoices.map((opt) =>
                $submissions.reduce(
                    (t, s) =>
                        t +
                        sGetChoice(s.data).option.filter((sOpt) => sOpt === opt)
                            .length,
                    0
                )
            );
        }

        return [];
    });
</script>

{#if $question !== undefined && qIsChoice($question.configuration)}
    <Chart
        options={{
            series: $question.configuration.choice.multi
                ? [
                      {
                          data: series(),
                      },
                  ]
                : series(),
            chart: {
                type: $question.configuration.choice.multi ? "bar" : "pie",
                height: 300,
            },
            labels: uniqueChoices,
        }}
    />
{/if}
