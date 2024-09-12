<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import { sGetPhoneNumber } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    $: callingCodesInUse = $submissions
        .map((s) => {
            return sGetPhoneNumber(s.data).calling_code.trim();
        })
        .filter((v, i, a) => v.trim().length > 0 && a.indexOf(v) === i);
    $: series = callingCodesInUse.map((callingCode) => {
        return $submissions.reduce((t, c) => {
            if (sGetPhoneNumber(c.data).calling_code.trim() === callingCode) {
                return t + 1;
            } else {
                return t;
            }
        }, 0);
    });
</script>

{#if $question !== undefined}
    <Chart
        options={{
            series: series,
            chart: {
                type: "pie",
                height: 300,
            },
            labels: callingCodesInUse,
        }}
    />
{/if}
