<script lang="ts">
    import { fade } from "svelte/transition";

    const genWidth = () => {
        const max = 100;
        const min = 60;
        return Math.floor(Math.random() * (max - min + 1) + min);
    };

    export let height: string | undefined = undefined;
    export let className = "";
    export let randomWidth = false;
    export let width: string = randomWidth ? `${genWidth()}%` : "100%";

    let show = false;
    setTimeout(() => (show = true), 500);
</script>

{#if show}
    <div
        class={`animate-pulse bg-primary-400/30 ${className} rounded-xl`}
        style:height
        style:width
        style:visibility={show ? "visible" : "hidden"}
        in:fade
    >
        <slot />
    </div>
{:else}
    <div style:height style:width={`${width}%`} />
{/if}
