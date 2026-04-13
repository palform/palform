<script lang="ts">
    import type { APIOrganisationInvitePreview } from "@palform/palform-typescript-openapi";
    import Main from "../../layouts/Main.svelte";
    import { APIs, humaniseAPIError } from "../../data/common";
    import { Alert, Button, Spinner } from "flowbite-svelte";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import { parseServerTime } from "../../data/util/time";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { showFailureToast, showToast } from "../../data/toast";
    import { faChampagneGlasses } from "@fortawesome/free-solid-svg-icons";
    import { navigate, p, route } from "../../router";

    interface Props {
        orgId?: string | undefined;
        inviteId?: string | undefined;
    }

    let { orgId, inviteId }: Props = $props();

    const orgIdResolved = $derived(orgId ?? route.params.orgId ?? "");
    const inviteIdResolved = $derived(inviteId ?? route.params.inviteId ?? "");

    let preview: APIOrganisationInvitePreview | undefined = $state();
    let previewLoading = $state(true);
    let previewError: string | undefined = $state(undefined);
    $effect(() => {
        APIs.orgInvites()
            .then((a) =>
                a.organisationInvitesPreview(orgIdResolved, inviteIdResolved)
            )
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
    });

    let acceptLoading = $state(false);
    let onInviteAccept = $derived(async () => {
        acceptLoading = true;
        try {
            await APIs.orgMembers().then((a) =>
                a.organisationMembersJoin(orgIdResolved, {
                    invite_id: inviteIdResolved,
                })
            );

            await showToast({
                label: `Welcome to ${preview!.org_display_name}!`,
                color: "green",
                icon: faChampagneGlasses,
            });
            void navigate(`/orgs/${orgIdResolved}/induction/member`);
        } catch (e) {
            await showFailureToast(e);
        }

        acceptLoading = false;
    });
</script>

<Main title="Join an organisation">
    {#if previewLoading}
        <div class="text-center">
            <Spinner size="12" />
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
                    onclick={onInviteAccept}
                    loading={acceptLoading}
                    disabled={acceptLoading}
                >
                    Accept invite
                </LoadingButton>
                <Button outline href={p("/")}>Go back home</Button>
            </div>
        </Alert>
    {/if}
</Main>
