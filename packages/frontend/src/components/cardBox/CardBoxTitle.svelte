<script lang="ts">
    import BrandedSpan from "../teams/brandings/BrandedSpan.svelte";
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<h2
    class={`text-primary-800 dark:text-primary-200 ${props.class ?? ""}`}
>
    <BrandedSpan sizeGroup="h2">
        {@render props.children?.()}
    </BrandedSpan>
</h2>
