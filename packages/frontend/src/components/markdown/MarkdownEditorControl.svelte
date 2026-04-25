<script lang="ts">
    import {
        FontAwesomeIcon,
        type FontAwesomeIconProps,
    } from "@fortawesome/svelte-fontawesome";
    import type { MouseEventHandler } from "svelte/elements";

    interface Props {
        icon: FontAwesomeIconProps["icon"];
        disabled?: boolean;
        onpress?: MouseEventHandler<HTMLButtonElement>;
    }

    let { icon, disabled = false, onpress }: Props = $props();
</script>

<button class="h-10 w-8 hover:bg-black/5" onclick={onpress} {disabled}>
    <FontAwesomeIcon {icon} />
</button>
