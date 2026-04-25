<script lang="ts">
    import type { Snippet } from "svelte";
    import BrandedSpan from "../components/teams/brandings/BrandedSpan.svelte";

    interface Props {
        className?: string;
        children?: Snippet;
    }

    let { className = "", children }: Props = $props();
</script>

<h1 class={`${className} dark:text-slate-200 font-display`}>
    <BrandedSpan sizeGroup="h1+">
        {@render children?.()}
    </BrandedSpan>
</h1>
