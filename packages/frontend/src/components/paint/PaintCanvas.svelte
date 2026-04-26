<script lang="ts">
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../data/contexts/brand";
    import { getStroke } from "perfect-freehand";
    import { getSvgPathFromStroke } from "../../data/util/painting";
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faDownload } from "@fortawesome/free-solid-svg-icons";

    const brandCtx = getBrandCtx();

    interface Props {
        points?: number[][][];
        readonly?: boolean;
        downloadButton?: boolean;
        onupdate?: (points: number[][][]) => void;
    }

    let {
        points = $bindable([]),
        readonly = false,
        downloadButton = false,
        onupdate,
    }: Props = $props();

    let onPointerDown = $derived((e: PointerEvent) => {
        if (readonly) return;
        const t = e.target as HTMLCanvasElement;
        t.setPointerCapture(e.pointerId);
        points = [...points, [[e.layerX, e.layerY, e.pressure]]];
        onupdate?.(points);
    });

    let onPointerMove = $derived((e: PointerEvent) => {
        if (readonly) return;
        if (e.buttons !== 1) return;
        points[points.length - 1] = [
            ...points[points.length - 1],
            [e.layerX, e.layerY, e.pressure],
        ];
        onupdate?.(points);
    });

    let strokes = $derived(
        points.map((e) =>
            getStroke(e, {
                size: 4,
                thinning: 0.5,
                smoothing: 0.5,
                streamline: 0.5,
            })
        )
    );

    let pathList = $derived(strokes.map((e) => getSvgPathFromStroke(e)));

    let onDownloadClick = $derived(() => {
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
    });
</script>

<div
    class="h-32 border-2 dark:border-gray-700 overflow-hidden"
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
>
    <svg
        onpointerdown={onPointerDown}
        onpointermove={onPointerMove}
        role="textbox"
        aria-label="Signature input"
        tabindex="0"
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
        onclick={onDownloadClick}
    >
        <FontAwesomeIcon icon={faDownload} class="me-3" />
        <span>
            Download signature
            <span class="block text-xs">SVG format</span>
        </span>
    </Button>
{/if}
