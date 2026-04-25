<script lang="ts">
    import { Chart } from "@flowbite-svelte-plugins/chart";
    import {
        getCorrelationContext,
        getFeatureGraphData,
    } from "../../../../data/contexts/analysis/correlation";
    import { lineToGraphPoints } from "../../../../data/util/lineToGraphPoints";

    interface Props {
        fromId: string;
        fromFeature: string;
        toId: string;
        toFeature: string;
    }

    let { fromId, fromFeature, toId, toFeature }: Props = $props();
    const correlationCtx = getCorrelationContext();

    const regressionResult = $derived(
        getFeatureGraphData(
            $correlationCtx,
            fromId,
            fromFeature,
            toId,
            toFeature
        )
    );

    let zippedCorrelations = $derived(
        regressionResult
            ? regressionResult.points[0].map((e, i) => {
                  return [e, regressionResult.points[1][i]];
              })
            : undefined
    );

    let linePoints = $derived(
        regressionResult
            ? lineToGraphPoints(
                  regressionResult.intercept,
                  regressionResult.gradient,
                  -10,
                  10
              )
            : undefined
    );
</script>

{#if zippedCorrelations && linePoints}
    <Chart
        options={{
            series: [
                {
                    type: "scatter",
                    data: zippedCorrelations,
                },
                {
                    type: "line",
                    data: linePoints,
                },
            ],
            chart: {
                type: "line",
                height: 350,
                zoom: { enabled: false },
            },
            fill: {
                type: "solid",
            },
            legend: {
                show: false,
            },
            markers: {
                size: [6, 0],
            },
            yaxis: {
                show: false,
            },
            xaxis: {
                tickAmount: 0,
                axisTicks: {
                    show: false,
                },
            },
            tooltip: {
                enabled: false,
            },
        }}
    />
{/if}
