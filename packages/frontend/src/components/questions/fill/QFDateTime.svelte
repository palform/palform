<script lang="ts">
    import {
        fillSendStore,
        setQuestionValue,
        sGetDateTime,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import DateTimePicker from "../../datePicker/DateTimePicker.svelte";
    import type { ConfigDateTime } from "@palform/palform-typescript-openapi";

    interface Props extends QuestionFillProps<ConfigDateTime> {}

    let { id, config, currentValue, onchange }: Props = $props();
    let value = $derived(currentValue ? sGetDateTime(currentValue).value : "");

    let onSubmissionUpdate = $derived((e: CustomEvent<string>) => {
        setQuestionValue(id, {
            DateTime: {
                value: e.detail,
            },
        });
        onchange();
    });
</script>

<DateTimePicker
    selectedDateTime={value === "" ? null : (value ?? null)}
    disabled={$fillSendStore?.loading}
    on:update={onSubmissionUpdate}
    class="mt-2"
    min={config.date_time.min ?? undefined}
    max={config.date_time.max ?? undefined}
    pickDate={config.date_time.collect_date}
    pickTime={config.date_time.collect_time}
/>
