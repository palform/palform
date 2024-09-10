<script lang="ts">
    import TextLogo from "../../assets/text_logo.svg";
    export let hideAlt = false;
</script>

<img
    src={TextLogo}
    alt={hideAlt ? "" : "Palform"}
    class={`w-full ${$$props.class}`}
/>
