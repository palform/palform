<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import type { APIQuestionConfigurationOneOf9 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import {
        fillSendStore,
        setQuestionValue,
        sGetDateTime,
    } from "../../../data/contexts/fill";
    import DateTimePicker from "../../datePicker/DateTimePicker.svelte";

    const dispatch = createEventDispatcher<{ change: undefined }>();

    export let id: string;
    export let config: APIQuestionConfigurationOneOf9;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue ? sGetDateTime(currentValue).value : "";

    $: onSubmissionUpdate = (e: CustomEvent<string>) => {
        setQuestionValue(id, {
            DateTime: {
                value: e.detail,
            },
        });
        dispatch("change");
    };
</script>

<DateTimePicker
    selectedDateTime={value === "" ? null : value}
    disabled={$fillSendStore?.loading}
    on:update={onSubmissionUpdate}
    class="mt-2"
    min={config.date_time.min ?? undefined}
    max={config.date_time.max ?? undefined}
    pickDate={config.date_time.collect_date}
    pickTime={config.date_time.collect_time}
/>
