<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import PaintCanvas from "../../../paint/PaintCanvas.svelte";
    import InfoText from "../../../type/InfoText.svelte";
    import { faCheckCircle } from "@fortawesome/free-solid-svg-icons";

    export let freeform: number[][][];
    export let initial: string;
    export let fullName: string;
    export let compact: boolean;
</script>

{#if compact}
    <p class="dark:text-gray-400">
        {#if freeform.length > 0 || initial.length > 0 || fullName.length > 0}
            <FontAwesomeIcon icon={faCheckCircle} class="text-green-500" />
            Signed
        {/if}
    </p>
{:else}
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
{/if}
