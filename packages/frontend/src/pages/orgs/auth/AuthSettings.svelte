<script lang="ts">
    import type { APIOrganisationAuthConfig } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import AuthSettingsForm from "../../../components/orgs/auth/AuthSettingsForm.svelte";
    import SkeletonPrimitive from "../../../components/SkeletonPrimitive.svelte";

    const orgCtx = getOrgContext();
    let config: APIOrganisationAuthConfig | null = null;
    let getConfigLoading = true;

    APIs.orgAuthConfig()
        .then((a) => a.organisationAuthConfigGet($orgCtx.org.id))
        .then((resp) => {
            config = resp.data;
        })
        .catch((e) => showFailureToast(e))
        .then(() => (getConfigLoading = false));
</script>

{#if getConfigLoading}
    <div class="space-y-6">
        <SkeletonPrimitive height="170px" />
        <SkeletonPrimitive height="60px" />
        <SkeletonPrimitive height="60px" />
        <SkeletonPrimitive height="60px" />
        <SkeletonPrimitive height="60px" />
        <SkeletonPrimitive height="60px" />
        <SkeletonPrimitive height="60px" />
        <SkeletonPrimitive height="40px" width="70px" />
    </div>
{:else}
    <AuthSettingsForm initialValue={config} />
{/if}
