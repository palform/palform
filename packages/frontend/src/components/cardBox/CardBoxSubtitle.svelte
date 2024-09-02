<script>
    import BrandedSpan from "../teams/brandings/BrandedSpan.svelte";
</script>

<p class={`text-primary-800/80 dark:text-primary-400 ${$$props.class ?? ""}`}>
    <BrandedSpan textLighter sizeGroup="p">
        <slot />
    </BrandedSpan>
</p>
