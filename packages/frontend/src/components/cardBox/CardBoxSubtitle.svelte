<script>
    import BrandedSpan from "../teams/brandings/BrandedSpan.svelte";
</script>

<p class={`text-slate-600 dark:text-slate-400 ${$$props.class ?? ""}`}>
    <BrandedSpan textLighter sizeGroup="p">
        <slot />
    </BrandedSpan>
</p>
