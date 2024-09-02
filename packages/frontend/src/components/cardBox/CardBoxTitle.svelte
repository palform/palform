<script lang="ts">
    import BrandedSpan from "../teams/brandings/BrandedSpan.svelte";
</script>

<h2
    class={`text-primary-800 dark:text-primary-200 ${$$props.class ?? ""}`}
>
    <BrandedSpan sizeGroup="h2">
        <slot />
    </BrandedSpan>
</h2>
