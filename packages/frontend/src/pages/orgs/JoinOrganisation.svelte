<script lang="ts">
    import type { APIOrganisationInvitePreview } from "@paltiverse/palform-typescript-openapi";
    import Main from "../../layouts/Main.svelte";
    import { APIs, humaniseAPIError } from "../../data/common";
    import { Alert, Button, Spinner } from "flowbite-svelte";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import { parseServerTime } from "../../data/util/time";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { showFailureToast, showToast } from "../../data/toast";
    import { faChampagneGlasses } from "@fortawesome/free-solid-svg-icons";
    import { navigate } from "svelte-routing";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    export let orgId: string;
    export let inviteId: string;

    let preview: APIOrganisationInvitePreview | undefined;
    let previewLoading = true;
    let previewError: string | undefined = undefined;
    $: APIs.orgInvites()
        .then((a) => a.organisationInvitesPreview(orgId, inviteId))
        .then((resp) => {
            preview = resp.data;
            previewError = undefined;
        })
        .catch((e) => {
            previewError = humaniseAPIError(e);
        })
        .finally(() => {
            previewLoading = false;
        });

    let acceptLoading = false;
    $: onInviteAccept = async () => {
        acceptLoading = true;
        try {
            await APIs.orgMembers().then((a) =>
                a.organisationMembersJoin(orgId, {
                    invite_id: inviteId,
                })
            );

            await showToast({
                label: `Welcome to ${preview!.org_display_name}!`,
                color: "green",
                icon: faChampagneGlasses,
            });
            navigate(`/orgs/${orgId}/induction/member`);
        } catch (e) {
            await showFailureToast(e);
        }

        acceptLoading = false;
    };
</script>

<Main title="Join an organisation">
    {#if previewLoading}
        <div class="text-center">
            <Spinner size={14} />
        </div>
    {/if}

    {#if previewError}
        <ErrorMsg e={previewError} targetDescriptor="invite" class="mt-4" />
    {/if}

    {#if preview}
        <Alert class="mt-4" border>
            <h2 class="text-lg">You're invited!</h2>

            <p>
                Your friends at <strong>{preview.org_display_name}</strong> have
                invited you to join their organisation.
            </p>
            <p>
                This invite expires {parseServerTime(
                    preview.expires_at
                ).toRelative()}.
            </p>

            <div class="mt-4 flex gap-x-2">
                <LoadingButton
                    on:click={onInviteAccept}
                    loading={acceptLoading}
                    disabled={acceptLoading}
                >
                    Accept invite
                </LoadingButton>
                <Button outline href="/" on:click={navigateEvent}>
                    Go back home
                </Button>
            </div>
        </Alert>
    {/if}
</Main>
