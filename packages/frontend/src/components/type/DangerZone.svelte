<script lang="ts">
    import { faSkull } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import InfoText from "./InfoText.svelte";
</script>

<h2 class={`text-xl text-red-500 font-bold ${$$props.class}`}>
    <FontAwesomeIcon icon={faSkull} />
    Danger Zone
</h2>
<InfoText class="text-sm">These actions may be irreversible.</InfoText>
