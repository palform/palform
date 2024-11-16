<script lang="ts">
    import { getBrandCtx } from "../../../data/contexts/brand";
    import { colorWithDeltaLightness } from "../../../data/util/color";

    const ctx = getBrandCtx();

    let colorList: {
        from: string;
        to: string;
        direction?: "bottom" | "right";
    }[] = [];

    $: (() => {
        if (!$ctx || !$ctx.background_color) {
            colorList = [];
            return;
        }

        if (!$ctx.background_color_accent) {
            colorList = [
                {
                    from: $ctx.background_color,
                    to: colorWithDeltaLightness($ctx.background_color, 0.8),
                },
            ];
        } else {
            colorList = [
                {
                    from: colorWithDeltaLightness($ctx.background_color, 1.2),
                    to: colorWithDeltaLightness(
                        $ctx.background_color,
                        0.9,
                        0.9
                    ),
                },
                {
                    from: colorWithDeltaLightness(
                        $ctx.background_color,
                        0.9,
                        0.8
                    ),
                    to: colorWithDeltaLightness(
                        $ctx.background_color_accent,
                        0.9,
                        0.8
                    ),
                },
                {
                    from: colorWithDeltaLightness(
                        $ctx.background_color_accent,
                        0.9,
                        0.6
                    ),
                    to: colorWithDeltaLightness(
                        $ctx.background_color,
                        0.9,
                        0.8
                    ),
                },
                {
                    from: colorWithDeltaLightness(
                        $ctx.background_color,
                        0.9,
                        0.8
                    ),
                    to: colorWithDeltaLightness($ctx.background_color, 1.2),
                },
            ];
        }
    })();
</script>

{#if colorList.length > 0}
    <div
        class="z-0 absolute top-0 left-0 w-full h-full overflow-hidden bg-white dark:bg-slate-900"
    >
        <div
            class={`w-full h-full grid ${colorList.length === 1 ? "grid-cols-1" : "grid-cols-2"} blur-[500px]`}
        >
            {#each colorList as color}
                <div
                    style:background-image={`linear-gradient(to ${color.direction ?? "right"}, ${color.from}, ${color.to})`}
                />
            {/each}
        </div>
    </div>
{/if}
