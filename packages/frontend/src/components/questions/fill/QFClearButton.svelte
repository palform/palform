<script lang="ts">
    import { t } from "../../../data/contexts/i18n";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        disabled?: boolean;
        class?: string;
        onclick?: MouseEventHandler<HTMLButtonElement>;
        hidden?: boolean;
        text?: string;
    }

    let {
        disabled = false,
        class: className,
        onclick,
        hidden,
        text = t("field_clear"),
    }: Props = $props();
</script>

<button
    class={`text-xs text-gray-600 hover:underline cursor-pointer ${hidden ? "invisible" : ""} ${className}`}
    type="button"
    {onclick}
    {disabled}
    aria-hidden={hidden}
>
    {text}
</button>
