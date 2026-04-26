<script lang="ts">
    import { Button, Input, Label, Modal, P } from "flowbite-svelte";
    import type { Snippet } from "svelte";
    import LoadingButton from "./LoadingButton.svelte";

    interface Props {
        show: boolean;
        targetName: string;
        confirmationWord?: string;
        children?: Snippet;
        ondelete: () => void | Promise<void>;
        loading?: boolean;
    }

    let {
        show = $bindable(),
        targetName,
        confirmationWord = "DELETE",
        children,
        ondelete,
        loading,
    }: Props = $props();

    let confirmationValue = $state("");
    let canDelete = $derived(confirmationWord === confirmationValue);

    let onDeleteClick = $derived(async () => {
        loading = true;
        await ondelete();
        loading = false;
    });
</script>

<Modal bind:open={show} title="Confirm deletion" permanent={loading}>
    <P>Are you sure you want to delete {targetName}?</P>
    {@render children?.()}

    <Label>
        Type <span>{confirmationWord}</span> to confirm
        <Input
            class="mt-2"
            bind:value={confirmationValue}
            autofocus
            color="red"
            required
            disabled={loading}
        />
    </Label>

    {#snippet footer()}
        <LoadingButton
            color="red"
            disabled={!canDelete || loading}
            {loading}
            onclick={onDeleteClick}
        >
            Delete
        </LoadingButton>
        <Button color="light" onclick={() => (show = false)}>Cancel</Button>
    {/snippet}
</Modal>
