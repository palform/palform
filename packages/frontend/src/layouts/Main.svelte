<script lang="ts">
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../data/contexts/brand";
    import { isInFrame } from "../data/util/iframe";
    import MainTitle from "./MainTitle.svelte";

    export let title: string | undefined = undefined;
    export let fluid = false;
    export let extraTight = false;
    export let fullHeight = false;
    export let verticalCenter = false;

    const brandCtx = getBrandCtx();
    $: showBrandImage =
        $brandCtx !== undefined && $brandCtx.background_image_asset_id != null;
    const isFrame = isInFrame();
</script>

<main
    class={`py-8 px-4 overflow-auto h-screen ${fluid ? "" : extraTight ? "md:px-20 lg:px-[15%] xl:px-[25%] 2xl:px-[30%]" : "md:px-20 lg:px-[15%]"} ${fullHeight ? "min-h-full relative" : ""} ${verticalCenter ? "h-screen flex" : ""} ${isFrame || showBrandImage ? "" : "bg-slate-50/50 dark:bg-slate-900"}`}
    style:align-items={verticalCenter ? "safe center" : undefined}
>
    <div
        class={`w-full ${showBrandImage ? "bg-slate-50 dark:bg-slate-900 p-8" : ""}`}
        style:border-radius={getRoundingAmountForBrand($brandCtx)}
    >
        {#if title !== undefined}
            <MainTitle className="mb-2">{title}</MainTitle>
        {/if}

        <div>
            <slot />
        </div>
    </div>
</main>
