<script lang="ts">
    import { faComment } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button } from "flowbite-svelte";
    import FeedbackMiniModal from "./FeedbackMiniModal.svelte";

    export let showMiniModal = false;
</script>

{#if !showMiniModal}
    <Button
        class="fixed bottom-4 right-4 z-10 shadow"
        size="sm"
        color="light"
        on:click={() => (showMiniModal = true)}
    >
        <FontAwesomeIcon icon={faComment} class="me-3 text-gray-500" />
        Give feedback
    </Button>
{:else}
    <FeedbackMiniModal on:done={() => (showMiniModal = false)} />
{/if}
