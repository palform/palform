<script lang="ts">
    import { Button, Modal } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import { showFailureToast, showSuccessToast } from "../../data/toast";

    export let open = false;
    const dispatch = createEventDispatcher<{ granted: undefined }>();

    const onEnableClick = async () => {
        const allowed = await navigator.storage.persist();
        if (!allowed) {
            await showFailureToast(
                "Failed to get storage permission; please try again."
            );
            return;
        }

        await showSuccessToast("Nice!");
        dispatch("granted");
    };
</script>

<Modal title="Enable browser storage" bind:open outsideclose>
    <p>We need permission to save your new key in your browser long-term.</p>
    <p>
        When you click the button below, your browser will prompt for
        permission. Please accept the prompt to continue.
    </p>

    <Button on:click={onEnableClick}>Enable storage</Button>
</Modal>
