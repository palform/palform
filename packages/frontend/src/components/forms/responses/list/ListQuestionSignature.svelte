<script lang="ts">
    import PaintCanvas from "../../../paint/PaintCanvas.svelte";
    import InfoText from "../../../type/InfoText.svelte";

    export let freeform: number[][][];
    export let initial: string;
    export let fullName: string;
</script>

<div class="mt-2">
    {#if freeform.length > 0}
        <PaintCanvas points={freeform} readonly downloadButton />
    {:else if initial.length > 0}
        <p
            class="font-mono font-medium text-3xl tracking-widest dark:text-gray-300"
        >
            {initial}
        </p>
    {:else}
        <InfoText>{fullName}</InfoText>
    {/if}
</div>
