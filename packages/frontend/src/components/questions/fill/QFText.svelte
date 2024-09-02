<script lang="ts">
    import type { APIQuestionConfigurationOneOf1 } from "@paltiverse/palform-typescript-openapi";
    import { Input, Textarea } from "flowbite-svelte";
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import { createEventDispatcher } from "svelte";
    import {
        fillSendStore,
        sGetText,
        setQuestionValue,
    } from "../../../data/contexts/fill";

    export let id: string;
    export let config: APIQuestionConfigurationOneOf1;
    export let currentValue: QuestionSubmissionData | undefined;

    const dispatch = createEventDispatcher<{ change: undefined }>();
    const onInput = (e: Event) => {
        if (currentValue === undefined) return;
        setQuestionValue(id, {
            Text: {
                value: (e.target as HTMLInputElement | HTMLTextAreaElement)
                    .value,
            },
        });
        dispatch("change");
    };

    $: value = currentValue ? sGetText(currentValue).value : "";
</script>

{#if config.text.is_long}
    <Textarea
        {id}
        disabled={$fillSendStore?.loading}
        on:input={onInput}
        {value}
    />
{:else}
    <Input
        {id}
        type={config.text.validator === "Email" ? "email" : "text"}
        disabled={$fillSendStore?.loading}
        on:input={onInput}
        {value}
    />
{/if}
