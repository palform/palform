<script lang="ts">
    import { fade } from "svelte/transition";

    const genWidth = () => {
        const max = 100;
        const min = 60;
        return Math.floor(Math.random() * (max - min + 1) + min);
    };

    interface Props {
        height?: string | undefined;
        className?: string;
        randomWidth?: boolean;
        width?: string;
        children?: import('svelte').Snippet;
    }

    let {
        height = undefined,
        className = "",
        randomWidth = false,
        width = randomWidth ? `${genWidth()}%` : "100%",
        children
    }: Props = $props();

    let show = $state(false);
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
        {@render children?.()}
    </div>
{:else}
    <div style:height style:width={`${width}%`}></div>
{/if}
