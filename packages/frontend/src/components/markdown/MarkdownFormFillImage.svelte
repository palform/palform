<script lang="ts">
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../data/contexts/brand";
    import { formFillStore } from "../../data/contexts/fill";

    export let href: string;
    const brandCtx = getBrandCtx();
</script>

{#if $formFillStore && $formFillStore.fillAccessToken}
    <div
        class="max-w-full lg:max-w-md my-3 overflow-hidden"
        style:border-radius={getRoundingAmountForBrand($brandCtx)}
    >
        <img
            role="presentation"
            alt="presentation"
            src={href.replaceAll("{{token}}", $formFillStore.fillAccessToken)}
            {...$$props}
        />
    </div>
{/if}
