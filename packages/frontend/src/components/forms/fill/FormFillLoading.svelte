<script lang="ts">
    import { fade } from "svelte/transition";
    import BusyProgressBar from "../../BusyProgressBar.svelte";
</script>

<div
    class="fixed top-0 left-0 z-50 w-screen h-screen bg-gray-50/50 dark:bg-slate-900 flex flex-col justify-center items-center"
    out:fade={{ duration: 300 }}
>
    <BusyProgressBar />
</div>
