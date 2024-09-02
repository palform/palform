<script lang="ts">
    import TeamAssetModal from "./TeamAssetModal.svelte";
    import { Input } from "flowbite-svelte";

    export let value: string | null = null;
    export let teamId: string;
    export let id: string;

    let showModal = false;
    $: onAssetSelect = (e: CustomEvent<string | null>) => {
        value = e.detail;
        showModal = false;
    };
</script>

<Input
    readonly
    class={$$props.class}
    value={value === null ? "Click to select..." : "Selected asset"}
    on:click={() => (showModal = true)}
    {id}
/>

<TeamAssetModal
    {teamId}
    allowClear
    bind:show={showModal}
    highlight={value ?? undefined}
    on:select={onAssetSelect}
/>
