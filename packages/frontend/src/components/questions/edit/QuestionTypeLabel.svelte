<script lang="ts">
    import type { APIQuestionConfiguration } from "@paltiverse/palform-typescript-openapi";
    import { humanQuestionTypeLabel } from "../../../data/contexts/formEditor";

    export let configuration: APIQuestionConfiguration;
</script>

<p class="text-slate-800/80 dark:text-slate-400">
    {humanQuestionTypeLabel(configuration)}
</p>
