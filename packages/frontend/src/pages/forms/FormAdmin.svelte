<script lang="ts">
    import { Progressbar, Spinner } from "flowbite-svelte";
    import { APIs } from "../../data/common";
    import FormResponseList from "../../components/forms/responses/list/FormResponseList.svelte";
    import { writable } from "svelte/store";
    import FormResponseOverview from "../../components/forms/responses/overview/FormResponseOverview.svelte";
    import FormEditor from "./FormEditor.svelte";
    import FormTokens from "./FormTokens.svelte";
    import { Route, Router } from "svelte-routing";
    import FormTabs from "../../components/forms/FormTabs.svelte";
    import MainTitle from "../../layouts/MainTitle.svelte";
    import FormSettings from "./FormSettings.svelte";
    import { getFormCtx, getOrgContext } from "../../data/contexts/orgLayout";
    import FormExport from "./FormExport.svelte";
    import { downloadSubmissionsForForm } from "../../data/crypto/results";
    import { onMount } from "svelte";
    import {
        type AnalysisCorrelationContext,
        initCorrelationContext,
        setCorrelationContext,
    } from "../../data/contexts/analysis/correlation";
    import { showFailureToast } from "../../data/toast";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faInfoCircle } from "@fortawesome/free-solid-svg-icons";
    import {
        setFormAdminContext,
        type FormAdminContext,
    } from "../../data/contexts/formAdmin";

    export let formId: string;
    const formAdminStore = writable<FormAdminContext>();
    setFormAdminContext(formAdminStore);

    const orgCtx = getOrgContext();

    let submissionsLoading = true;
    let submissionsTracker = writable<
        { total: number; done: number } | undefined
    >(undefined);
    let submissionsTerminateHandle = writable<boolean>(false);

    const doSubmissionLoad = (formId: string) => {
        submissionsLoading = true;
        submissionsTracker.set(undefined);

        submissionsTerminateHandle.set(false);
        downloadSubmissionsForForm(
            $orgCtx.org.id,
            formId,
            submissionsTracker,
            formAdminStore,
            submissionsTerminateHandle
        )
            .then(() => (submissionsLoading = false))
            .catch(showFailureToast);
    };

    $: doSubmissionLoad(formId);

    const correlationCtx = writable<AnalysisCorrelationContext>({
        manager: null,
        correlations: new Map(),
    });
    setCorrelationContext(correlationCtx);

    onMount(() => {
        return () => {
            submissionsTerminateHandle.set(true);
        };
    });

    let formLoading = true;
    $: APIs.questions()
        .then((a) => a.questionsList($orgCtx.org.id, formId))
        .then((resp) => {
            $formAdminStore.formId = formId;
            $formAdminStore.questions = resp.data;
            formLoading = false;
        });

    $: {
        if (!submissionsLoading && !formLoading) {
            initCorrelationContext(
                formId,
                $formAdminStore.questions,
                $formAdminStore.submissions,
                correlationCtx
            );
        }
    }

    let groupsLoading = true;
    $: APIs.questionGroups()
        .then((a) => a.questionGroupsList($orgCtx.org.id, formId))
        .then((resp) => {
            $formAdminStore.groups = resp.data;
            groupsLoading = false;
        });

    let tokensLoading = true;
    $: APIs.fillTokens()
        .then((a) => a.fillAccessTokensList($orgCtx.org.id, formId))
        .then((resp) => {
            $formAdminStore.tokens = resp.data;
            tokensLoading = false;
        });

    $: formCtx = getFormCtx(formId);
</script>

{#if formLoading || submissionsLoading || groupsLoading || tokensLoading || $formAdminStore === undefined || $formCtx === undefined}
    <div class="text-center">
        {#if $submissionsTracker !== undefined}
            <Progressbar
                progress={($submissionsTracker.done /
                    $submissionsTracker.total) *
                    100}
            />
            <p class="text-center text-gray-400 dark:text-gray-500 mt-2">
                Decrypting {$submissionsTracker.done}/{$submissionsTracker.total}
            </p>
            <p class="text-center text-gray-400 dark:text-gray-500 mt-2">
                <FontAwesomeIcon icon={faInfoCircle} class="me-2" />
                We're caching your decrypted submissions so this is faster next time
            </p>
        {:else}
            <Spinner size={14} />
        {/if}
    </div>
{:else}
    <MainTitle className="mb-4">
        {$formCtx.editor_name}
    </MainTitle>

    <Router>
        <FormTabs />
        <Route path="/" component={FormResponseOverview} />
        <Route path="/responses" component={FormResponseList} />
        <Route path="/edit" component={FormEditor} />
        <Route path="/tokens" component={FormTokens} />
        <Route path="/export" component={FormExport} />
        <Route path="/settings" component={FormSettings} />
    </Router>
{/if}
