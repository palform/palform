<script lang="ts">
    import { writable } from "svelte/store";
    import {
        setBrandCtx,
        type BrandContext,
    } from "../../../data/contexts/brand";

    interface Props {
        ctx: BrandContext | null | undefined;
        children?: import("svelte").Snippet;
    }

    let { ctx, children }: Props = $props();
    const writableCtx = writable<BrandContext | undefined>(
        // svelte-ignore state_referenced_locally
        ctx ? ctx : undefined
    );
    setBrandCtx(writableCtx);

    $effect(() => {
        writableCtx.set(ctx ? ctx : undefined);
    });
</script>

{@render children?.()}
