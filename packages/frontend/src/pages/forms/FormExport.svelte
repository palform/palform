<script lang="ts">
    import { Alert, Button, Label, Select, Toggle } from "flowbite-svelte";
    import {
        exportFormSubmissions,
        exportFormats,
        type ExportSubmissionsConfig,
    } from "../../data/export";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { showFailureToast } from "../../data/toast";
    import { isEntitled } from "../../data/billing/entitlement";
    import { navigate } from "../../router";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { getFormAdminContext } from "../../data/contexts/formAdmin";

    const formAdminCtx = getFormAdminContext();
    const orgCtx = getOrgContext();
    const entitled = isEntitled("export");
    let format: ExportSubmissionsConfig["format"] = $state("CSV");
    let useQuestionIDs = $state(false);
    let useSectionIDs = $state(false);

    let loading = $state(false);
    let onExportClick = $derived(async () => {
        loading = true;

        try {
            exportFormSubmissions($formAdminCtx, {
                format,
                use_question_ids: useQuestionIDs,
                use_group_ids: useSectionIDs,
            });
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });

    let onBillingClick = $derived(() => {
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
    });
</script>

{#if !$entitled}
    <Alert border>
        <h3 class="text-lg">Export submissions securely from Palform</h3>
        <p>
            Convert your submissions to CSV or JSON in a single click. The data
            never leaves your browser!
        </p>
        <p>To continue, please upgrade your plan.</p>

        <Button class="mt-2" onclick={onBillingClick}>Continue</Button>
    </Alert>
{:else}
    <form class="space-y-4">
        <Label>
            Format
            <Select class="mt-1" items={exportFormats} bind:value={format} />
        </Label>

        <Label>
            <Toggle bind:checked={useQuestionIDs}>
                Use question IDs instead of question titles
            </Toggle>
        </Label>
        <Label>
            <Toggle bind:checked={useSectionIDs}>
                Use section IDs instead of section titles
            </Toggle>
        </Label>

        <LoadingButton disabled={loading} {loading} onclick={onExportClick}>
            Export
        </LoadingButton>
    </form>
{/if}
