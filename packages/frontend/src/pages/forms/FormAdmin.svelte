<script lang="ts">
    import { Progressbar, Spinner } from "flowbite-svelte";
    import { APIs } from "../../data/common";
    import { writable } from "svelte/store";
    import FormTabs from "../../components/forms/FormTabs.svelte";
    import { getFormCtx, getOrgContext } from "../../data/contexts/orgLayout";
    import { downloadSubmissionsForForm } from "../../data/crypto/results";
    import { onMount, type Snippet } from "svelte";
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
    import FormTitleClickable from "../../components/forms/FormTitleClickable.svelte";
    import { route } from "../../router";

    interface Props {
        children?: Snippet;
    }

    let { children }: Props = $props();

    const orgId = $derived(route.params.orgId ?? "");
    const formId = $derived(route.params.formId ?? "");
    const formAdminStore = writable<FormAdminContext>();
    setFormAdminContext(formAdminStore);

    const orgCtx = getOrgContext();

    let submissionsLoading = $state(true);
    let submissionsTracker = writable<
        { total: number; done: number } | undefined
    >(undefined);
    let submissionsTerminateHandle = writable<boolean>(false);

    const doSubmissionLoad = (orgId: string, formId: string) => {
        submissionsLoading = true;
        submissionsTracker.set(undefined);

        submissionsTerminateHandle.set(false);
        downloadSubmissionsForForm(
            orgId,
            formId,
            submissionsTracker,
            formAdminStore,
            submissionsTerminateHandle
        )
            .then(() => (submissionsLoading = false))
            .catch(showFailureToast);
    };

    // Depend only on route params, not `$orgCtx`. Updating form metadata (e.g. settings save)
    // mutates the org store and must not re-run submission download or the whole page flashes.
    $effect(() => {
        doSubmissionLoad(orgId, formId);
    });

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

    let formLoading = $state(true);
    $effect(() => {
        let _formId = formId;
        APIs.questions()
            .then((a) => a.questionsList($orgCtx.org.id, _formId))
            .then((resp) => {
                $formAdminStore.formId = formId;
                $formAdminStore.questions = resp.data;
                formLoading = false;
            });
    });

    $effect(() => {
        if (!submissionsLoading && !formLoading) {
            initCorrelationContext(
                formId,
                $formAdminStore.questions,
                $formAdminStore.submissions,
                correlationCtx
            );
        }
    });

    let groupsLoading = $state(true);
    $effect(() => {
        let _formId = formId;
        APIs.questionGroups()
            .then((a) => a.questionGroupsList($orgCtx.org.id, _formId))
            .then((resp) => {
                $formAdminStore.groups = resp.data;
                groupsLoading = false;
            });
    });

    let tokensLoading = $state(true);
    $effect(() => {
        let _formId = formId;
        APIs.fillTokens()
            .then((a) => a.fillAccessTokensList($orgCtx.org.id, _formId))
            .then((resp) => {
                $formAdminStore.tokens = resp.data;
                tokensLoading = false;
            });
    });

    let formCtx = $derived(getFormCtx(formId));
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
            <Spinner size="8" />
        {/if}
    </div>
{:else}
    <FormTitleClickable class="mb-4" />

    <FormTabs />

    <div class="mt-4">
        {@render children?.()}
    </div>
{/if}
