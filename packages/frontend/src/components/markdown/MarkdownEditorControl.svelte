<script lang="ts">
    import {
        FontAwesomeIcon,
        type FontAwesomeIconProps,
    } from "@fortawesome/svelte-fontawesome";

    export let icon: FontAwesomeIconProps["icon"];
    export let disabled = false;
</script>

<button class="h-10 w-8 hover:bg-black/5" on:click {disabled}>
    <FontAwesomeIcon {icon} />
</button>
