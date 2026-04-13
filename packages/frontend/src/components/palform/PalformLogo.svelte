<script lang="ts">
    import TextLogo from "../../assets/text_logo.svg";

    interface Props {
        hideAlt?: boolean;
        class?: string;
    }

    let { hideAlt = false, class: className }: Props = $props();
</script>

<img
    src={TextLogo}
    alt={hideAlt ? "" : "Palform"}
    class={`w-full ${className ?? ""}`}
/>
