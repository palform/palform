<script lang="ts">
    import { Helper, Input, Label } from "flowbite-svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import type { APIAdminUser } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import TextButton from "../../TextButton.svelte";
    import LoadingButton from "../../LoadingButton.svelte";

    let profile: APIAdminUser | undefined = $state();
    let displayName: string | undefined = $state();
    let loading = $state(true);

    APIs.authWithToken()
        .then((a) => a.authTest())
        .then((resp) => {
            profile = resp.data.user;
            displayName = profile.display_name ?? undefined;
        })
        .catch(showFailureToast)
        .finally(() => {
            loading = false;
        });

    let saveLoading = $state(false);
    let onSave = $derived(async (e: Event) => {
        e.preventDefault();
        if (displayName === undefined) return;

        saveLoading = true;
        try {
            await APIs.adminUsers().then((a) =>
                a.adminUsersUpdate({
                    display_name: displayName || null,
                })
            );
            await showSuccessToast("Saved");
        } catch (e) {
            await showFailureToast(e);
        }

        saveLoading = false;
    });
</script>

<SectionHeading>Your profile</SectionHeading>

{#if loading}
    <SkeletonPrimitive height="35px" className="mt-4" />
    <SkeletonPrimitive height="35px" className="mt-4" />
    <SkeletonPrimitive height="35px" className="mt-4" />
    <SkeletonPrimitive height="35px" width="100px" className="mt-4" />
{:else if profile}
    <form class="space-y-4 mt-4" onsubmit={onSave}>
        <Label>
            User ID
            <Input
                class="mt-1"
                value={profile.id}
                readonly
                disabled={saveLoading}
            />
        </Label>

        <Label>
            Email address
            <Input
                class="mt-1"
                value={profile.email}
                readonly
                type="email"
                disabled={saveLoading}
            />
            <Helper class="mt-1">
                If you need to change this, please <TextButton
                    class="text-xs inline"
                    href="https://palform.app/support"
                    >contact support</TextButton
                >.
            </Helper>
        </Label>

        <Label>
            Display name
            <Input
                class="mt-1"
                bind:value={displayName}
                disabled={saveLoading}
            />
            <Helper class="mt-1">
                This is visible to others in your organisations.
            </Helper>
        </Label>

        <LoadingButton
            type="submit"
            disabled={saveLoading}
            loading={saveLoading}
        >
            Save
        </LoadingButton>
    </form>
{/if}
