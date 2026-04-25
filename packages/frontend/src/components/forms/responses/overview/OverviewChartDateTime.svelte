<script lang="ts">
    import { DateTime } from "luxon";
    import { sGetDateTime } from "../../../../data/contexts/fill";
    import { labelForQuestionDate } from "../../../../data/util/time";
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsDateTime } from "../../../../data/contexts/formEditor";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let question = $derived(ctxGetQuestion(questionId));
    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    let questionConfig = $derived(
        $question
            ? qIsDateTime($question.configuration)
                ? $question.configuration.date_time
                : undefined
            : undefined
    );

    let labels = $derived(
        $submissions
            .map((e) => sGetDateTime(e.data).value)
            .filter((e) => !!e && e.length > 0)
            .map((e) => DateTime.fromISO(e!))
            .toSorted((a, b) => a.toMillis() - b.toMillis())
            .map((e) => {
                return labelForQuestionDate(questionConfig!, e);
            })
    );

    let labelSet = $derived(Array.from(new Set(labels)));
    let labelCounts = $derived(
        labelSet.map((l) =>
            labels.reduce((t, c) => {
                if (c === l) {
                    return t + 1;
                } else {
                    return t;
                }
            }, 0)
        )
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
