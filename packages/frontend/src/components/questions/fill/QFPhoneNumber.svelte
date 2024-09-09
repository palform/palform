<script lang="ts">
    import type { APICountryWithCallingCode } from "@paltiverse/palform-typescript-openapi";
    import {
        setQuestionValue,
        sGetPhoneNumber,
    } from "../../../data/contexts/fill";
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import { createEventDispatcher } from "svelte";
    import { Input } from "flowbite-svelte";
    import CallingCodeDropdown from "../../callingCode/CallingCodeDropdown.svelte";
    import { t } from "../../../data/contexts/i18n";

    const dispatch = createEventDispatcher<{ change: undefined }>();
    export let id: string;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue
        ? sGetPhoneNumber(currentValue)
        : { calling_code: "", number: "" };

    $: onCallingCodeSelect = (e: CustomEvent<APICountryWithCallingCode>) => {
        if (currentValue === undefined) return;

        e.preventDefault();
        setQuestionValue(id, {
            PhoneNumber: {
                calling_code: `+${e.detail.calling_code}`,
                number: value.number,
            },
        });
        dispatch("change");
    };

    $: onNumberInput = (e: Event) => {
        if (currentValue === undefined) return;

        const t = (e.target as HTMLInputElement).value;
        setQuestionValue(id, {
            PhoneNumber: {
                calling_code: value.calling_code,
                number: t,
            },
        });
        dispatch("change");
    };
</script>

<div class="flex gap-2">
    <div>
        <CallingCodeDropdown
            bind:value={value.calling_code}
            on:update={onCallingCodeSelect}
        />
    </div>
    <Input
        disabled={value.calling_code === ""}
        type="tel"
        value={value.number}
        on:input={onNumberInput}
        placeholder={value.calling_code === ""
            ? t("phone_pls_choose_country")
            : t("phone_enter_number")}
    />
</div>
