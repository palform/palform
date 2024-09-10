<script lang="ts">
    import { fade } from "svelte/transition";
    import BusyProgressBar from "../../BusyProgressBar.svelte";
    import PalformLogo from "../../palform/PalformLogo.svelte";
</script>

<div
    class="fixed top-0 left-0 z-50 w-screen h-screen bg-gray-100 dark:bg-slate-900 flex flex-col gap-4 justify-center items-center"
    out:fade={{ duration: 300 }}
>
    <PalformLogo class="h-8" hideAlt={true} />
    <BusyProgressBar />
</div>
