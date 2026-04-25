<script lang="ts">
    import {
        Alert,
        Button,
        ButtonGroup,
        Input,
        InputAddon,
        Label,
        Toggle,
    } from "flowbite-svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import { navigate } from "../../../router";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import LoadingButton from "../../LoadingButton.svelte";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import type { SetSubmissionAutoDeleteRequest } from "@palform/palform-typescript-openapi";

    const entitled = isEntitled("submission_auto_delete");
    const orgCtx = getOrgContext();
    const formMetadataCtx = getFormCtx();

    const onBillingContinueClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
    };

    let autoDeleteToggle = $state(
        !!$formMetadataCtx.auto_delete_submission_after_days
    );
    let newDayCount = $state(
        $formMetadataCtx.auto_delete_submission_after_days ?? 30
    );
    let loading = $state(false);
    let onToggleChange = $derived(async () => {
        loading = true;
        const newValue: SetSubmissionAutoDeleteRequest = {
            days: autoDeleteToggle ? 30 : null,
        };
        try {
            await APIs.forms().then((a) =>
                a.formsSetAutoDelete(
                    $orgCtx.org.id,
                    $formMetadataCtx.id,
                    newValue
                )
            );
            updateFormCtx(orgCtx, $formMetadataCtx.id, (ctx) => {
                ctx.auto_delete_submission_after_days = newValue.days;
            });
            newDayCount = newValue.days ?? 30;
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    });

    let onSaveClick = $derived(async () => {
        loading = true;
        try {
            await APIs.forms().then((a) =>
                a.formsSetAutoDelete($orgCtx.org.id, $formMetadataCtx.id, {
                    days: newDayCount,
                })
            );
            await showSuccessToast("Saved");
            updateFormCtx(orgCtx, $formMetadataCtx.id, (ctx) => {
                ctx.auto_delete_submission_after_days = newDayCount;
            });
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    });
</script>

<SectionHeading class="mb-4">Submission auto-deletion</SectionHeading>

{#if !$entitled}
    <Alert border>
        <h3 class="text-lg">Please upgrade to access this feature</h3>
        <p>
            Automatically delete your old responses, making data retention
            compliance easier than ever!
        </p>
        <Button class="mt-2" onclick={onBillingContinueClick}>Continue</Button>
    </Alert>
{:else}
    <Toggle
        bind:checked={autoDeleteToggle}
        onchange={onToggleChange}
        disabled={loading}
    >
        Automatically delete responses after some time
    </Toggle>

    {#if autoDeleteToggle}
        <Label class="mt-4">
            Delete submissions older than
            <div class="mt-1">
                <ButtonGroup>
                    <Input type="number" bind:value={newDayCount} />
                    <InputAddon>Days</InputAddon>
                </ButtonGroup>
            </div>
        </Label>

        {#if newDayCount !== $formMetadataCtx.auto_delete_submission_after_days}
            <LoadingButton
                buttonClass="mt-4"
                {loading}
                disabled={loading}
                onclick={onSaveClick}
            >
                Save
            </LoadingButton>
        {/if}
    {/if}
{/if}
