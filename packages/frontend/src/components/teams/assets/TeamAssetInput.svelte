<script lang="ts">
    import TeamAssetModal from "./TeamAssetModal.svelte";
    import { Input } from "flowbite-svelte";

    interface Props {
        value?: string | null;
        teamId: string;
        id: string;
        class?: string;
    }

    let {
        value = $bindable(null),
        teamId,
        id,
        class: className,
    }: Props = $props();

    let showModal = $state(false);
    const onAssetSelect = (e: string | null) => {
        value = e;
        showModal = false;
    };
</script>

<Input
    readonly
    class={className}
    value={value === null ? "Click to select..." : "Selected asset"}
    onclick={() => (showModal = true)}
    {id}
/>

<TeamAssetModal
    {teamId}
    allowClear
    bind:show={showModal}
    highlight={value ?? undefined}
    onselect={onAssetSelect}
/>
