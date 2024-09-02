<script lang="ts">
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../data/contexts/brand";
    import MainTitle from "./MainTitle.svelte";

    export let title: string | undefined = undefined;
    export let fluid = false;
    export let extraTight = false;
    export let fullHeight = false;

    const brandCtx = getBrandCtx();
    $: showBrandImage =
        $brandCtx !== undefined && $brandCtx.background_image_asset_id != null;
</script>

<main
    class={`py-8 px-4 ${fluid ? "" : extraTight ? "md:px-20 lg:px-[15%] xl:px-[25%] 2xl:px-[30%]" : "md:px-20 lg:px-[15%]"} ${fullHeight ? "min-h-full relative" : ""}`}
>
    <div
        class={showBrandImage ? "bg-slate-50 dark:bg-slate-900 p-8 " : ""}
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
