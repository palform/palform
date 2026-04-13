<script lang="ts">
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import { sGetScale } from "../../../../data/contexts/fill";
    import { genScaleList } from "../../../../data/util/scaleList";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsScale } from "../../../../data/contexts/formEditor";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let question = $derived(ctxGetQuestion(questionId));
    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    let series: () => number[] = $derived(() => {
        if ($question !== undefined && qIsScale($question.configuration)) {
            return genScaleList(
                $question.configuration.scale.min,
                $question.configuration.scale.max
            ).map(
                (scaleVal) =>
                    $submissions.filter((s) => {
                        return sGetScale(s.data).value === scaleVal;
                    }).length
            );
        }

        return [];
    });
</script>

{#if $question !== undefined && qIsScale($question.configuration)}
    <Chart
        options={{
            series: [
                {
                    data: series(),
                    name: "Responses",
                },
            ],
            chart: {
                type: "bar",
                height: 300,
            },
            labels: genScaleList(
                $question.configuration.scale.min,
                $question.configuration.scale.max
            ).map((e) => e.toString()),
        }}
    />
{/if}
