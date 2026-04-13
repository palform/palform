<script lang="ts">
    import BrandedSpan from "../teams/brandings/BrandedSpan.svelte";
    interface Props {
        children?: import('svelte').Snippet;
        [key: string]: any
    }

    let { ...props }: Props = $props();
</script>

<p class={`text-slate-600 dark:text-slate-400 ${props.class ?? ""}`}>
    <BrandedSpan textLighter sizeGroup="p">
        {@render props.children?.()}
    </BrandedSpan>
</p>
