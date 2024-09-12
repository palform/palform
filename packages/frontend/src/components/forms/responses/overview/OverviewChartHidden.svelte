<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import { sGetHidden } from "../../../../data/contexts/fill";
    import { ctxSubmissionsForQuestion } from "../../../../data/contexts/formAdmin";

    export let questionId: string;

    $: submissions = ctxSubmissionsForQuestion(questionId);

    $: values = $submissions
        .map((e) => sGetHidden(e.data).value)
        .filter((e) => e.length > 0);
    $: uniqueValues = Array.from(new Set(values));
    $: counts = uniqueValues.map((v) => {
        return values.reduce((t, c) => {
            if (c === v) {
                return t + 1;
            } else {
                return t;
            }
        }, 0);
    });
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
