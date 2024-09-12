<script lang="ts">
    import type { APIQuestionGroupStepStrategyJumpCaseCondition } from "@paltiverse/palform-typescript-openapi";
    import { Button } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faTrash } from "@fortawesome/free-solid-svg-icons";
    import { createEventDispatcher } from "svelte";
    import { matcherLabel } from "../../../../data/util/stepStrategyConditions";
    import { getFormAdminContext } from "../../../../data/contexts/formAdmin";

    export let condition: APIQuestionGroupStepStrategyJumpCaseCondition;
    export let showDelete = false;

    const formAdminCtx = getFormAdminContext();

    $: question = $formAdminCtx.questions.find(
        (e) => e.id === condition.question_id
    );

    const dispatch = createEventDispatcher<{ delete: undefined }>();
</script>

{#if question}
    <div class={`py-2 px-4 rounded-md flex justify-between ${$$props.class}`}>
        <p class="text-gray-800 dark:text-gray-300">
            <span class="font-medium">{question.title}</span>
            <span class="text-sm block">{matcherLabel(condition.matcher)}</span>
        </p>

        {#if showDelete}
            <Button
                size="sm"
                color="red"
                outline
                on:click={() => dispatch("delete")}
            >
                <FontAwesomeIcon icon={faTrash} />
            </Button>
        {/if}
    </div>
{/if}
