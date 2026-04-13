<script lang="ts">
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import { sGetHidden } from "../../../../data/contexts/fill";
    import { ctxSubmissionsForQuestion } from "../../../../data/contexts/formAdmin";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    let values = $derived(
        $submissions
            .map((e) => sGetHidden(e.data).value)
            .filter((e) => e.length > 0)
    );
    let uniqueValues = $derived(Array.from(new Set(values)));
    let counts = $derived(
        uniqueValues.map((v) => {
            return values.reduce((t, c) => {
                if (c === v) {
                    return t + 1;
                } else {
                    return t;
                }
            }, 0);
        })
    );
</script>

<Chart
    options={{
        series: [
            {
                name: "",
                data: counts,
            },
        ],
        labels: uniqueValues,
        chart: {
            type: "bar",
            height: 350,
        },
    }}
/>
