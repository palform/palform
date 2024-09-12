<script lang="ts">
    import { Alert, Button } from "flowbite-svelte";
    import { navigateEvent } from "../../../../utils/navigate";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../../data/contexts/orgLayout";
    import OverviewGroup from "./OverviewGroup.svelte";
    import { submissionIsError } from "../../../../data/crypto/results";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faWarning } from "@fortawesome/free-solid-svg-icons";
    import {
        getFormAdminContext,
        getGroupTitle,
    } from "../../../../data/contexts/formAdmin";
    import { qIsInfo } from "../../../../data/contexts/formEditor";

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();
    const formMetadataCtx = getFormCtx();
    $: groupedQuestions = $formAdminCtx.groups.map((g) => ({
        group: g,
        questions: $formAdminCtx.questions
            .filter((q) => !qIsInfo(q.configuration) && q.group_id === g.id)
            .map((q) => q.id),
    }));

    $: hasSomeFailure = $formAdminCtx.submissions.some((e) =>
        submissionIsError(e)
    );
</script>

<ol class="space-y-4">
    {#if groupedQuestions.length === 0}
        <Alert border>
            <p class="text-lg">Welcome to your new form!</p>
            <p>
                To get started, create some questions. Then, create a Share
                Token so others can fill your form.
            </p>

            <Button
                class="mt-2"
                size="sm"
                href={`/orgs/${$orgCtx.org.id}/forms/${$formAdminCtx.formId}/edit`}
                on:click={navigateEvent}
            >
                Edit form
            </Button>
        </Alert>
    {/if}

    {#if hasSomeFailure}
        <Alert border color="yellow">
            <svelte:fragment slot="icon">
                <FontAwesomeIcon icon={faWarning} />
            </svelte:fragment>
            <h2 class="text-lg">Failed to decrypt some responses</h2>
            <p>
                We were unable to decrypt at least one response. Check the
                "Responses" tab for more info.
            </p>
        </Alert>
    {/if}

    {#each groupedQuestions as group (group.group.id)}
        <li>
            <OverviewGroup
                groupTitle={getGroupTitle(
                    $formMetadataCtx.one_question_per_page,
                    $formAdminCtx,
                    group.group
                )}
                questions={group.questions}
            />
        </li>
    {/each}
</ol>
