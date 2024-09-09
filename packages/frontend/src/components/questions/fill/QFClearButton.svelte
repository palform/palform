<script lang="ts">
    import { t } from "../../../data/contexts/i18n";

    export let disabled = false;
</script>

<button
    class={`text-xs text-gray-600 hover:underline cursor-pointer ${$$props.class}`}
    type="button"
    on:click
    {disabled}
>
    {t("field_clear")}
</button>
