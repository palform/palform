<script lang="ts">
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../data/contexts/brand";
    import { getStroke } from "perfect-freehand";
    import { getSvgPathFromStroke } from "../../data/util/painting";
    import { createEventDispatcher } from "svelte";
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faDownload } from "@fortawesome/free-solid-svg-icons";

    const brandCtx = getBrandCtx();

    const dispatch = createEventDispatcher<{ update: number[][][] }>();
    export let points: number[][][] = [];
    export let readonly = false;
    export let downloadButton = false;

    $: onPointerDown = (e: PointerEvent) => {
        if (readonly) return;
        const t = e.target as HTMLCanvasElement;
        t.setPointerCapture(e.pointerId);
        points = [...points, [[e.layerX, e.layerY, e.pressure]]];
        dispatch("update", points);
    };

    $: onPointerMove = (e: PointerEvent) => {
        if (readonly) return;
        if (e.buttons !== 1) return;
        points[points.length - 1] = [
            ...points[points.length - 1],
            [e.layerX, e.layerY, e.pressure],
        ];
        dispatch("update", points);
    };

    $: strokes = points.map((e) =>
        getStroke(e, {
            size: 4,
            thinning: 0.5,
            smoothing: 0.5,
            streamline: 0.5,
        })
    );

    $: pathList = strokes.map((e) => getSvgPathFromStroke(e));

    $: onDownloadClick = () => {
        const svg = document.createElementNS(
            "http://www.w3.org/2000/svg",
            "svg"
        );
        svg.setAttribute("fill", "black");
        svg.setAttribute("stroke", "none");

        for (const pathData of pathList) {
            const icon = document.createElementNS(
                "http://www.w3.org/2000/svg",
                "path"
            );
            icon.setAttribute("d", pathData);
            svg.appendChild(icon);
        }

        const blob = new Blob([svg.outerHTML]);
        const url = URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = "signature.svg";
        link.click();
        URL.revokeObjectURL(url);
    };
</script>

<div
    class="h-32 border-2 dark:border-gray-700 overflow-hidden"
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
>
    <svg
        on:pointerdown={onPointerDown}
        on:pointermove={onPointerMove}
        class="h-full w-full dark:fill-white"
    >
        {#each pathList as pathData}
            <path d={pathData} />
        {/each}
    </svg>
</div>

{#if downloadButton && pathList.length > 0}
    <Button
        size="sm"
        color="light"
        class="mt-4 text-left flex"
        on:click={onDownloadClick}
    >
        <FontAwesomeIcon icon={faDownload} class="me-3" />
        <span>
            Download signature
            <span class="block text-xs">SVG format</span>
        </span>
    </Button>
{/if}
