<script lang="ts">
    import { Alert, Button } from "flowbite-svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { navigateEvent } from "../../../utils/navigate";
    import AuthSettings from "./AuthSettings.svelte";
    import AuthSettingsIntro from "../../../components/orgs/auth/AuthSettingsIntro.svelte";

    const entitled = isEntitled("oidc");
    const orgCtx = getOrgContext();
</script>

{#if !$entitled}
    <AuthSettingsIntro />
{:else if !$orgCtx.org.subdomain}
    <Alert border>
        <h2 class="text-lg">Set up a subdomain</h2>
        <p>
            To proceed with authentication configuration, you'll need to create
            a subdomain. This will be used both as a place for your users to
            sign in, as well as a clear shortened link for your forms.
        </p>
        <Button
            class="mt-2"
            href={`/orgs/${$orgCtx.org.id}/settings/subdomain`}
            on:click={navigateEvent}
        >
            Continue
        </Button>
    </Alert>
{:else}
    <AuthSettings />
{/if}
