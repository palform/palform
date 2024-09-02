<script>
    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";
    import PalformLogo from "../../components/palform/PalformLogo.svelte";
    let pulseLogo = false;
    let stillLoading = false;
    onMount(() => {
        setTimeout(() => {
            pulseLogo = true;
        }, 500);
        setTimeout(() => {
            stillLoading = true;
        }, 5000);
    });
</script>

<div
    class="fixed z-10 bg-slate-50/50 dark:bg-slate-900 w-screen h-screen flex justify-center items-center"
    transition:fade={{ duration: 200 }}
>
    <div class="animate-pulse relative">
        {#if pulseLogo}
            <PalformLogo />
        {/if}
        {#if stillLoading}
            <p class="absolute w-full text-center text-gray-400 mt-2" in:slide>
                Still loading...
            </p>
        {/if}
    </div>
</div>
