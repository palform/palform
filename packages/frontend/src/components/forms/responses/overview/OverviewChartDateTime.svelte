<script lang="ts">
    import { DateTime } from "luxon";
    import { sGetDateTime } from "../../../../data/contexts/fill";
    import { labelForQuestionDate } from "../../../../data/util/time";
    import { Chart } from "flowbite-svelte";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsDateTime } from "../../../../data/contexts/formEditor";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    $: questionConfig = $question
        ? qIsDateTime($question.configuration)
            ? $question.configuration.date_time
            : undefined
        : undefined;

    $: labels = $submissions
        .map((e) => sGetDateTime(e.data).value)
        .filter((e) => !!e && e.length > 0)
        .map((e) => DateTime.fromISO(e!))
        .toSorted((a, b) => a.toMillis() - b.toMillis())
        .map((e) => {
            return labelForQuestionDate(questionConfig!, e);
        });

    $: labelSet = Array.from(new Set(labels));
    $: labelCounts = labelSet.map((l) =>
        labels.reduce((t, c) => {
            if (c === l) {
                return t + 1;
            } else {
                return t;
            }
        }, 0)
    );
</script>

<Chart
    options={{
        series: [
            {
                data: labelCounts,
                name: "",
            },
        ],
        chart: {
            type: "bar",
            height: 350,
        },
        labels: labelSet,
    }}
/>
