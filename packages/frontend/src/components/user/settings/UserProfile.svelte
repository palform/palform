<script lang="ts">
    import { Helper, Input, Label } from "flowbite-svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import type { APIAdminUser } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import TextButton from "../../TextButton.svelte";
    import LoadingButton from "../../LoadingButton.svelte";

    let profile: APIAdminUser | undefined;
    let loading = true;
    APIs.authWithToken()
        .then((a) => a.authTest())
        .then((resp) => {
            profile = resp.data.user;
        })
        .catch(showFailureToast)
        .finally(() => {
            loading = false;
        });

    let saveLoading = false;
    $: onSave = async (e: Event) => {
        e.preventDefault();
        if (profile === undefined) return;

        saveLoading = true;
        try {
            await APIs.adminUsers().then((a) =>
                a.adminUsersUpdate({
                    display_name: profile!.display_name,
                })
            );
            await showSuccessToast("Saved");
        } catch (e) {
            await showFailureToast(e);
        }

        saveLoading = false;
    };
</script>

<SectionHeading>Your profile</SectionHeading>

{#if loading}
    <SkeletonPrimitive height="35px" className="mt-4" />
    <SkeletonPrimitive height="35px" className="mt-4" />
    <SkeletonPrimitive height="35px" className="mt-4" />
    <SkeletonPrimitive height="35px" width="100px" className="mt-4" />
{:else if profile}
    <form class="space-y-4 mt-4" on:submit={onSave}>
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
                bind:value={profile.display_name}
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
