<script lang="ts">
    import { Modal } from "flowbite-svelte";
    import Captcha from "../../captcha/Captcha.svelte";

    interface Props {
        open: boolean;
        oncomplete: (captchaResult: string) => void;
        onclear?: () => void;
    }

    let { open = $bindable(), oncomplete, onclear }: Props = $props();
</script>

<Modal bind:open title="One last step...">
    <p>Please verify your identity to submit this form.</p>
    <Captcha oncomplete={(e) => oncomplete(e)} onclear={() => onclear?.()} />
</Modal>
