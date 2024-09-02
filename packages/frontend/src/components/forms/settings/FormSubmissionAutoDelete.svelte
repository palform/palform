<script lang="ts">
    import {
        Alert,
        Button,
        ButtonGroup,
        InputAddon,
        Label,
        NumberInput,
        Toggle,
    } from "flowbite-svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import { navigate } from "svelte-routing";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import LoadingButton from "../../LoadingButton.svelte";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import type { SetSubmissionAutoDeleteRequest } from "@paltiverse/palform-typescript-openapi";

    const entitled = isEntitled("submission_auto_delete");
    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    const onBillingContinueClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
    };

    let autoDeleteToggle = !!$formCtx.auto_delete_submission_after_days;
    let newDayCount = $formCtx.auto_delete_submission_after_days ?? 30;
    let loading = false;
    $: onToggleChange = async () => {
        loading = true;
        const newValue: SetSubmissionAutoDeleteRequest = {
            days: autoDeleteToggle ? 30 : null,
        };
        try {
            await APIs.forms().then((a) =>
                a.formsSetAutoDelete($orgCtx.org.id, $formCtx.id, newValue)
            );
            updateFormCtx(orgCtx, $formCtx.id, (ctx) => {
                ctx.auto_delete_submission_after_days = newValue.days;
            });
            newDayCount = newValue.days ?? 30;
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };

    $: onSaveClick = async () => {
        loading = true;
        try {
            await APIs.forms().then((a) =>
                a.formsSetAutoDelete($orgCtx.org.id, $formCtx.id, {
                    days: newDayCount,
                })
            );
            await showSuccessToast("Saved");
            updateFormCtx(orgCtx, $formCtx.id, (ctx) => {
                ctx.auto_delete_submission_after_days = newDayCount;
            });
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<SectionHeading class="mb-4">Submission auto-deletion</SectionHeading>

{#if !$entitled}
    <Alert border>
        <h3 class="text-lg">Please upgrade to access this feature</h3>
        <p>
            Automatically delete your old responses, making data retention
            compliance easier than ever!
        </p>
        <Button class="mt-2" on:click={onBillingContinueClick}>Continue</Button>
    </Alert>
{:else}
    <Toggle
        bind:checked={autoDeleteToggle}
        on:change={onToggleChange}
        disabled={loading}
    >
        Automatically delete responses after some time
    </Toggle>

    {#if autoDeleteToggle}
        <Label class="mt-4">
            Delete submissions older than
            <div class="mt-1">
                <ButtonGroup>
                    <NumberInput type="number" bind:value={newDayCount} />
                    <InputAddon>Days</InputAddon>
                </ButtonGroup>
            </div>
        </Label>

        {#if newDayCount !== $formCtx.auto_delete_submission_after_days}
            <LoadingButton
                buttonClass="mt-4"
                {loading}
                disabled={loading}
                on:click={onSaveClick}
            >
                Save
            </LoadingButton>
        {/if}
    {/if}
{/if}
