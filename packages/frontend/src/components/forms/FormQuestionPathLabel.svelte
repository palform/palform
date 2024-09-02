<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { ctxGetGroup, ctxGetQuestion } from "../../data/contexts/results";
    import { faChevronRight } from "@fortawesome/free-solid-svg-icons";

    export let questionId: string;
    export let featureName: string | undefined = undefined;
    $: question = ctxGetQuestion(questionId);
    $: group = $question ? ctxGetGroup($question.group_id) : undefined;
</script>

{#if $question && $group}
    {$group.title ?? `Section ${$group.position + 1}`}
    <FontAwesomeIcon icon={faChevronRight} class="me-1 ms-1" />
    <strong>{$question.title}</strong>

    {#if featureName}
        <FontAwesomeIcon icon={faChevronRight} class="me-1 ms-1" />
        {featureName}
    {/if}
{/if}
