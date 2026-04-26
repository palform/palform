<script lang="ts">
    import type {
        APICountryWithCallingCode,
        ConfigPhoneNumber,
    } from "@palform/palform-typescript-openapi";
    import {
        setQuestionValue,
        sGetPhoneNumber,
        type QuestionFillProps,
    } from "../../../data/contexts/fill";
    import { Input } from "flowbite-svelte";
    import CallingCodeDropdown from "../../callingCode/CallingCodeDropdown.svelte";
    import { t } from "../../../data/contexts/i18n";

    interface Props extends QuestionFillProps<ConfigPhoneNumber> {}

    let { id, currentValue, onchange, config }: Props = $props();
    let value = $derived(
        currentValue
            ? sGetPhoneNumber(currentValue)
            : { calling_code: "", number: "" }
    );

    let onCallingCodeSelect = $derived((e: APICountryWithCallingCode) => {
        if (currentValue === undefined) return;

        setQuestionValue(id, {
            PhoneNumber: {
                calling_code: `+${e.calling_code}`,
                number: value.number,
            },
        });
        onchange();
    });

    let onNumberInput = $derived((e: Event) => {
        if (currentValue === undefined) return;

        const t = (e.target as HTMLInputElement).value;
        setQuestionValue(id, {
            PhoneNumber: {
                calling_code: value.calling_code,
                number: t,
            },
        });
        onchange();
    });
</script>

<div class="flex gap-2">
    <div>
        <CallingCodeDropdown
            bind:value={value.calling_code}
            onupdate={onCallingCodeSelect}
            allowedValues={config.phone_number.allowed_calling_codes}
        />
    </div>
    <Input
        disabled={value.calling_code === ""}
        type="tel"
        value={value.number}
        oninput={onNumberInput}
        placeholder={value.calling_code === ""
            ? t("phone_pls_choose_country")
            : t("phone_enter_number")}
    />
</div>
