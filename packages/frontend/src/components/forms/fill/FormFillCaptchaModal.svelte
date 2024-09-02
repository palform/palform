<script lang="ts">
    import { Modal } from "flowbite-svelte";
    import Captcha from "../../captcha/Captcha.svelte";
    import { createEventDispatcher } from "svelte";

    export let open: boolean;

    const dispatch = createEventDispatcher<{
        complete: string;
        clear: undefined;
    }>();
</script>

<Modal bind:open title="One last step...">
    <p>Please verify your identity to submit this form.</p>
    <Captcha
        on:complete={(e) => dispatch("complete", e.detail)}
        on:clear={() => dispatch("clear")}
    />
</Modal>
