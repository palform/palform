<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../data/contexts/brand";
    import { formFillStore } from "../../data/contexts/fill";

    interface Props extends HTMLAttributes<HTMLImageElement> {
        href: string;
    }

    let { href, ...props }: Props = $props();
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
            {...props}
        />
    </div>
{/if}
