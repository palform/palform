<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import { qIsScale } from "../../../../data/contexts/questionsEditing";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/results";
    import { sGetScale } from "../../../../data/contexts/fill";
    import { genScaleList } from "../../../../data/util/scaleList";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    let series: number[] = [];
    $: {
        if ($question !== undefined && qIsScale($question.configuration)) {
            series = genScaleList(
                $question.configuration.scale.min,
                $question.configuration.scale.max
            ).map(
                (scaleVal) =>
                    $submissions.filter((s) => {
                        return sGetScale(s.data).value === scaleVal;
                    }).length
            );
        }
    }
</script>

{#if $question !== undefined && qIsScale($question.configuration)}
    <Chart
        options={{
            series: [
                {
                    data: series,
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
