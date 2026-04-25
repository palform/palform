<script lang="ts">
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import { sGetPhoneNumber } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let question = $derived(ctxGetQuestion(questionId));
    let submissions = $derived(ctxSubmissionsForQuestion(questionId));

    let callingCodesInUse = $derived(
        $submissions
            .map((s) => {
                return sGetPhoneNumber(s.data).calling_code.trim();
            })
            .filter((v, i, a) => v.trim().length > 0 && a.indexOf(v) === i)
    );
    let series = $derived(
        callingCodesInUse.map((callingCode) => {
            return $submissions.reduce((t, c) => {
                if (
                    sGetPhoneNumber(c.data).calling_code.trim() === callingCode
                ) {
                    return t + 1;
                } else {
                    return t;
                }
            }, 0);
        })
    );
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
