<script lang="ts">
    import { Button, Modal } from "flowbite-svelte";
    import { showFailureToast, showSuccessToast } from "../../data/toast";

    interface Props {
        open?: boolean;
        ongranted: () => void;
    }

    let { open = $bindable(false), ongranted }: Props = $props();

    const onEnableClick = async () => {
        const allowed = await navigator.storage.persist();
        if (!allowed) {
            await showFailureToast(
                "Failed to get storage permission; please try again."
            );
            return;
        }

        await showSuccessToast("Nice!");
        ongranted();
    };
</script>

<Modal title="Enable browser storage" bind:open outsideclose>
    <p>
        We need permission to save your encryption key in your browser
        long-term.
    </p>
    <p>
        When you click the button below, your browser will prompt for
        permission. Please accept the prompt to continue.
    </p>

    <Button onclick={onEnableClick}>Enable storage</Button>
</Modal>
