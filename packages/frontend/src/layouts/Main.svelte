<script lang="ts">
    import {
        getBrandCtx,
        getBrandIsNonNeutralBackground,
        getRoundingAmountForBrand,
    } from "../data/contexts/brand";
    import { isInFrame } from "../data/util/iframe";
    import MainTitle from "./MainTitle.svelte";

    interface Props {
        title?: string | undefined;
        fluid?: boolean;
        extraTight?: boolean;
        fullHeight?: boolean;
        verticalCenter?: boolean;
        children?: import('svelte').Snippet;
    }

    let {
        title = undefined,
        fluid = false,
        extraTight = false,
        fullHeight = false,
        verticalCenter = false,
        children
    }: Props = $props();

    const brandCtx = getBrandCtx();
    let isNonNeutralBg = $derived(getBrandIsNonNeutralBackground($brandCtx));
    const isFrame = isInFrame();
</script>

<main
    class={`py-8 px-4 overflow-auto h-screen ${fluid ? "" : extraTight ? "md:px-20 lg:px-[15%] xl:px-[25%] 2xl:px-[30%]" : "md:px-20 lg:px-[15%]"} ${fullHeight ? "min-h-full relative" : ""} ${verticalCenter ? "h-screen flex" : ""} ${isFrame || isNonNeutralBg ? "" : "bg-slate-50/50 dark:bg-slate-900"}`}
    style:align-items={verticalCenter ? "safe center" : undefined}
>
    <div
        class="w-full"
        style:border-radius={getRoundingAmountForBrand($brandCtx)}
    >
        {#if title !== undefined}
            <MainTitle className="mb-2">{title}</MainTitle>
        {/if}

        <div>
            {@render children?.()}
        </div>
    </div>
</main>
