<script lang="ts">
    import type { APIQuestionConfiguration } from "@palform/palform-typescript-openapi";
    import { humanQuestionTypeLabel } from "../../../data/contexts/formEditor";

    interface Props {
        configuration: APIQuestionConfiguration;
    }

    let { configuration }: Props = $props();
</script>

<p class="text-slate-800/80 dark:text-slate-400">
    {humanQuestionTypeLabel(configuration)}
</p>
