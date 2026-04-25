<script lang="ts">
    import { sGetAddress, sIsNonEmpty } from "../../../../data/contexts/fill";
    import { ctxSubmissionsForQuestion } from "../../../../data/contexts/formAdmin";
    import OpenLayersMap from "../../../map/OpenLayersMap.svelte";

    interface Props {
        questionId: string;
    }

    let { questionId }: Props = $props();

    let submissions = $derived(ctxSubmissionsForQuestion(questionId));
    let heatmapPoints = $derived($submissions
        .filter((s) => sIsNonEmpty(s.data))
        .map((s) => {
            const { point } = sGetAddress(s.data);
            return [point.lat, point.lng] as [number, number];
        }));
</script>

<OpenLayersMap {heatmapPoints} />
