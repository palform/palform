<script lang="ts">
    import { writable } from "svelte/store";
    import {
        setBrandCtx,
        type BrandContext,
    } from "../../../data/contexts/brand";

    export let ctx: BrandContext | null | undefined;
    const writableCtx = writable<BrandContext | undefined>(
        ctx ? ctx : undefined,
    );
    setBrandCtx(writableCtx);

    $: writableCtx.set(ctx ? ctx : undefined);
</script>

<slot />
