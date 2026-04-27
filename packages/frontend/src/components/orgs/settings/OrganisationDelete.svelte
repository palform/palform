<script lang="ts">
    import { Alert, Checkbox, Label, P, Spinner } from "flowbite-svelte";
    import { APIs } from "../../../data/common";
    import {
        getOrgContext,
        reloadGlobalAlert,
    } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import DestructiveModal from "../../DestructiveModal.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import CancelPlanReasonSelect from "../../billing/manage/CancelPlanReasonSelect.svelte";
    import type {
        APIOrganisationDeletionRequest,
        APIOrganisationManifest,
        CancelPlanRequestReason,
    } from "@palform/palform-typescript-openapi";
    import OrganisationManifest from "./OrganisationManifest.svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { signOut } from "../../../data/auth";
    import { typedNavigate } from "../../../router";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();

    let existingRequestsLoading = $state(true);
    let latestPendingRequest: APIOrganisationDeletionRequest | null =
        $state(null);
    $effect(() => {
        let orgId = $orgCtx.org.id;

        (async () => {
            const resp = await APIs.orgDeletionRequests().then((a) =>
                a.organisationDeletionRequestsList(orgId)
            );

            latestPendingRequest = null;
            for (const request of resp.data) {
                if (request.status === "GracePeriod") {
                    latestPendingRequest = request;
                    break;
                }
            }

            existingRequestsLoading = false;
        })();
    });

    let showDeleteModal = $state(false);
    let manifestLoading = $state(false);
    let reason: CancelPlanRequestReason = $state("Other");
    let includeOwnAccount = $state(false);
    let manifest: APIOrganisationManifest | null = $state(null);
    let onStartSubmit = $derived(async (e: SubmitEvent) => {
        e.preventDefault();

        reason;
        includeOwnAccount;
        manifestLoading = true;

        const response = await APIs.orgs().then((a) =>
            a.orgsDelete($orgCtx.org.id, {
                reason,
                dry_run: true,
                include_own_account: includeOwnAccount,
            })
        );
        manifest = response.data;

        for (const subscription of manifest.active_subscriptions) {
            if (subscription.canceling_at_end) {
                continue;
            }

            await showFailureToast(
                "Please cancel your active subscription first."
            );
            manifestLoading = false;
            return;
        }

        manifestLoading = false;
        showDeleteModal = true;
    });

    let onDelete = $derived(async () => {
        reason;
        includeOwnAccount;
        const orgId = $orgCtx.org.id;
        try {
            await APIs.orgs().then((a) =>
                a.orgsDelete(orgId, {
                    reason,
                    include_own_account: includeOwnAccount,
                    dry_run: false,
                })
            );
            await showSuccessToast("Organisation deletion requested");
            await reloadGlobalAlert(orgCtx);
            showDeleteModal = false;
        } catch (e) {
            await showFailureToast(e);
        }
        manifestLoading = false;
    });

    let cancelLoading = $state(false);
    let onCancel = $derived(async () => {
        if (latestPendingRequest === null) return;
        const orgId = $orgCtx.org.id;

        cancelLoading = true;
        try {
            await APIs.orgDeletionRequests().then((a) =>
                a.organisationDeletionRequestsCancel(
                    orgId,
                    latestPendingRequest!.id
                )
            );
            await showSuccessToast(
                "Deletion cancelled! Your organisation will no longer be deleted."
            );
            latestPendingRequest = null;
        } catch (e) {
            await showFailureToast(e);
        }
        cancelLoading = false;
    });

    let skipLoading = $state(false);
    let onSkip = $derived(async () => {
        if (latestPendingRequest === null) return;
        const orgId = $orgCtx.org.id;

        skipLoading = true;
        try {
            await APIs.orgDeletionRequests().then((a) =>
                a.organisationDeletionRequestsSkip(
                    orgId,
                    latestPendingRequest!.id
                )
            );

            if (latestPendingRequest.include_user) {
                await showSuccessToast(
                    "Your organisation and user account have been permanently deleted. Thanks for using Palform :)"
                );
                await signOut();
                await typedNavigate("/auth/login");
            } else {
                await showSuccessToast(
                    "Your organisation has been deleted. Thanks for using Palform :)"
                );
                await typedNavigate("/");
            }
        } catch (e) {
            await showFailureToast(e);
            skipLoading = false;
        }
    });
</script>

<DestructiveModal
    bind:show={showDeleteModal}
    targetName={$orgCtx.org.display_name}
    confirmationWord={$orgCtx.org.display_name}
    ondelete={onDelete}
>
    {#if manifest !== null}
        <P>
            This will delete all resources belonging to your organisation,
            including:
        </P>

        <OrganisationManifest {manifest} />

        {#if includeOwnAccount}
            <P>Your user account will also be deleted.</P>
        {/if}

        <P>
            Everything will be <strong>permanently deleted</strong> following a 24-hour
            grace period. You can cancel the deletion at any time before the end of
            this period. You can also skip the grace period entirely after starting
            it.
        </P>
    {/if}
</DestructiveModal>

<Alert color="red" class={className}>
    <h2 class="text-lg">
        {#if latestPendingRequest === null}
            Delete your organisation
        {:else}
            Organisation pending deletion
        {/if}
    </h2>

    {#if existingRequestsLoading}
        <Spinner color="gray" size="12" class="mt-4" />
    {:else if latestPendingRequest === null}
        <p>
            This will delete <strong>everything</strong> in your organisation, including
            all forms, responses, audit logs, etc.
        </p>
        <p>Make sure any active subscription is cancelled before deleting.</p>
        <p>
            Please click the button below to begin the process. We'll show you a
            list of what will be deleted. You can then confirm your choice, and
            your resources will be deleted <strong
                >after a 24 hour grace period</strong
            >.
        </p>
        <p>
            During the grace period, you can cancel the pending deletion at any
            time. You can also choose to skip the grace period after submitting
            your request.
        </p>

        <form onsubmit={onStartSubmit}>
            <Label class="mt-4">
                Reason for deletion
                <CancelPlanReasonSelect class="mt-2" bind:value={reason} />
            </Label>

            <fieldset class="mt-4">
                <Checkbox bind:checked={includeOwnAccount}>
                    Also delete my user account
                </Checkbox>
            </fieldset>

            {#if includeOwnAccount}
                <Alert class="mt-4">
                    Your user account and login credentials will be deleted. Any
                    other organisations you are a member of <strong
                        >will not be deleted</strong
                    >. If you want to delete them, please do so separately
                    before deleting your user account.
                </Alert>
            {/if}

            <LoadingButton
                loading={manifestLoading}
                disabled={manifestLoading}
                buttonClass="mt-4 mb-2"
                color="red"
                type="submit"
            >
                Continue
            </LoadingButton>
        </form>

        <p>Your organisation won't be deleted yet.</p>
    {:else}
        <p>
            Your organisation is <strong>pending deletion</strong> and will be
            deleted
            <strong>
                {parseServerTime(latestPendingRequest.deletion_at).toRelative()}
            </strong>.
        </p>
        {#if latestPendingRequest.include_user}
            <p>
                <strong>Your user account will also be deleted</strong> as per your
                request.
            </p>
        {/if}
        <p>
            All of your organisation's resources will be permanently deleted.
            After the end of the grace period, this cannot be undone.
        </p>

        <div class="mt-4">
            <LoadingButton
                color="red"
                loading={skipLoading}
                disabled={skipLoading || cancelLoading}
                onclick={onSkip}
            >
                Delete immediately
            </LoadingButton>
            <LoadingButton
                color="light"
                loading={cancelLoading}
                disabled={cancelLoading || skipLoading}
                onclick={onCancel}
            >
                Cancel deletion
            </LoadingButton>
        </div>
    {/if}
</Alert>
