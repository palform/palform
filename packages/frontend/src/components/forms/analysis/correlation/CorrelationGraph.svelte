<script lang="ts">
    import { Chart } from "flowbite-svelte";
    import {
        getCorrelationContext,
        getFeatureGraphData,
    } from "../../../../data/contexts/analysis/correlation";
    import { lineToGraphPoints } from "../../../../data/util/lineToGraphPoints";

    export let fromId: string;
    export let fromFeature: string;
    export let toId: string;
    export let toFeature: string;
    const correlationCtx = getCorrelationContext();

    const regressionResult = getFeatureGraphData(
        $correlationCtx,
        fromId,
        fromFeature,
        toId,
        toFeature
    );

    $: zippedCorrelations = regressionResult
        ? regressionResult.points[0].map((e, i) => {
              return [e, regressionResult.points[1][i]];
          })
        : undefined;

    $: linePoints = regressionResult
        ? lineToGraphPoints(
              regressionResult.intercept,
              regressionResult.gradient,
              -10,
              10
          )
        : undefined;
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
