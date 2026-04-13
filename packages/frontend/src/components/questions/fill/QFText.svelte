<script lang="ts">
    import { Input, Textarea } from "flowbite-svelte";
    import {
        fillSendStore,
        sGetText,
        setQuestionValue,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import type { ConfigText } from "@palform/palform-typescript-openapi";

    interface Props extends QuestionFillProps<ConfigText> {}

    let { id, config, currentValue, onchange }: Props = $props();

    const onInput = (e: Event) => {
        if (currentValue === undefined) return;
        setQuestionValue(id, {
            Text: {
                value: (e.target as HTMLInputElement | HTMLTextAreaElement)
                    .value,
            },
        });
        onchange();
    };

    let value = $derived(currentValue ? sGetText(currentValue).value : "");
</script>

{#if config.text.is_long}
    <Textarea
        {id}
        class="w-full"
        disabled={$fillSendStore?.loading}
        oninput={onInput}
        {value}
    />
{:else}
    <Input
        {id}
        type={config.text.validator === "Email" ? "email" : "text"}
        disabled={$fillSendStore?.loading}
        oninput={onInput}
        {value}
    />
{/if}
