<script lang="ts">
    import { sGetAddress, sIsNonEmpty } from "../../../../data/contexts/fill";
    import { ctxSubmissionsForQuestion } from "../../../../data/contexts/formAdmin";
    import OpenLayersMap from "../../../map/OpenLayersMap.svelte";

    export let questionId: string;

    $: submissions = ctxSubmissionsForQuestion(questionId);
    $: heatmapPoints = $submissions
        .filter((s) => sIsNonEmpty(s.data))
        .map((s) => {
            const { point } = sGetAddress(s.data);
            return [point.lat, point.lng] as [number, number];
        });
</script>

<OpenLayersMap {heatmapPoints} />
