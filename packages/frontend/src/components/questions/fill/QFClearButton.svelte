<script lang="ts">
    import { t } from "../../../data/contexts/i18n";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        disabled?: boolean;
        class?: string;
        onclick?: MouseEventHandler<HTMLButtonElement>;
        hidden?: boolean;
        text?: string;
        // The background of the link is lighter than normal when viewed in dark mode
        lighterInDarkMode?: boolean;
    }

    let {
        disabled = false,
        class: className,
        onclick,
        hidden,
        text = t("field_clear"),
        lighterInDarkMode,
    }: Props = $props();
</script>

<button
    class={`text-xs hover:underline cursor-pointer ${hidden ? "invisible" : ""} ${lighterInDarkMode ? "dark:text-gray-300 text-gray-600" : "text-gray-600"} ${className}`}
    type="button"
    {onclick}
    {disabled}
    aria-hidden={hidden}
>
    {text}
</button>
