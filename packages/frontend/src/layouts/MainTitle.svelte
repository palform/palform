<script lang="ts">
    import BrandedSpan from "../components/teams/brandings/BrandedSpan.svelte";

    export let className = "";
</script>

<h1 class={`${className} dark:text-slate-200 font-display`}>
    <BrandedSpan sizeGroup="h1+">
        <slot />
    </BrandedSpan>
</h1>
