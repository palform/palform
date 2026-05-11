<script lang="ts">
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import { sGetRank } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsChoice, qIsRank } from "../../../../data/contexts/formEditor";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let question = $derived(ctxGetQuestion(questionId));
    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    let uniqueChoices = $derived([
        ...new Set($submissions.flatMap((e) => sGetRank(e.data).value)),
    ]);

    const series: { name: string; data: number[] }[] = $derived.by(() => {
        if ($question === undefined || qIsChoice($question.configuration)) {
            return [];
        }

        const rankList = [...Array(uniqueChoices.length).keys()];

        return rankList.map((rank) => {
            return {
                name: `Rank ${rank + 1}`,
                data: uniqueChoices.map((opt) => {
                    return $submissions.reduce((prev, c) => {
                        const val = sGetRank(c.data).value;
                        if (rank > val.length - 1) return prev;
                        if (val[rank] !== opt) return prev;
                        return prev + 1;
                    }, 0);
                }),
            };
        });
    });
</script>

{#if $question !== undefined && qIsRank($question.configuration)}
    <Chart
        options={{
            series,
            chart: {
                type: "bar",
                height: 300,
                stacked: true,
                stackType: "100%",
            },
            labels: uniqueChoices,
        }}
    />
{/if}
