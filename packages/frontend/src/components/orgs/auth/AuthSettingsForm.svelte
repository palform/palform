<script lang="ts">
    import type { APIOrganisationAuthConfig } from "@paltiverse/palform-typescript-openapi";
    import {
        Alert,
        Button,
        Helper,
        Input,
        Label,
        Modal,
        Toggle,
    } from "flowbite-svelte";
    import TextButton from "../../../components/TextButton.svelte";
    import LoadingButton from "../../../components/LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { navigateEvent } from "../../../utils/navigate";

    export let initialValue: APIOrganisationAuthConfig | null;
    const orgCtx = getOrgContext();

    let discoveryUrl = initialValue?.oidc_discovery_url ?? "";
    let clientId = initialValue?.client_id ?? "";
    let clientSecret = initialValue?.client_secret ?? "";
    let teamFieldName = initialValue?.team_mapping_field_name ?? "";
    let revokeTeamMappings = initialValue?.revoke_team_mappings ?? false;

    let showSecret = false;
    $: onSecretToggle = (e: Event) => {
        e.preventDefault();
        showSecret = !showSecret;
    };

    let loading = false;
    let newRegistrationComplete = false;
    $: onSubmit = async (e: Event) => {
        e.preventDefault();
        loading = true;
        const request: APIOrganisationAuthConfig = {
            oidc_discovery_url: discoveryUrl,
            client_id: clientId,
            client_secret: clientSecret,
            team_mapping_field_name: teamFieldName || null,
            revoke_team_mappings: revokeTeamMappings,
        };

        try {
            await APIs.orgAuthConfig().then((a) =>
                a.organisationAuthConfigPut($orgCtx.org.id, request)
            );
            await showSuccessToast("Configuration saved!");
            if (initialValue === null) {
                newRegistrationComplete = true;
            }
            initialValue = request;
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };

    $: onNewDomainContinueClick = () => {
        window.location.href = `https://${$orgCtx.org.subdomain ?? ""}.palform.app/auth/login`;
    };
</script>

<Modal
    bind:open={newRegistrationComplete}
    title="OIDC set up complete!"
    dismissable={false}
>
    <p>You have successfully set up OIDC on your account.</p>
    <p>From now on, your organisation lives at:</p>
    <p class="font-display text-2xl">
        https://{$orgCtx.org.subdomain ?? ""}.palform.app
    </p>
    <p>
        To continue, you'll need to visit this site and sign in using OIDC. Make
        sure your OIDC email address matches your current Palform email address.
    </p>
    <Button slot="footer" on:click={onNewDomainContinueClick}>Continue</Button>
</Modal>

{#if initialValue === null}
    <Alert border class="mb-8 space-y-2">
        <h2 class="text-lg">Set up OpenID Connect</h2>
        <p>
            OpenID Connect is a series of protocols for authenticating with an
            external service, such as your organisation's central login (e.g.
            Microsoft/Google Workspace). Most widely-used identity platforms
            support it, and it's really easy to set up.
        </p>
        <p>
            <strong>Currently, OIDC is not set up.</strong> Once you set it up, existing
            email-only users will be asked to connect their accounts. New users will
            only be able to sign in with OIDC.
        </p>
        <p>
            If an OIDC user doesn't exist in Palform, an account will be created
            automatically. <strong
                >This may count against your plan's user quota.</strong
            >
        </p>
        <p>OIDC users cannot join other organisations.</p>
        <p>
            OIDC currently cannot be turned off through our dashboard, but we
            can help you with it if it's needed.
        </p>
    </Alert>
{:else}
    <Alert border class="mb-8 space-y-2" color="green">
        <h2 class="text-lg">OpenID Connect is active</h2>
        <p>Users now sign into your organisation via OpenID Connect.</p>
        <p>
            Please use <strong
                >https://{$orgCtx.org.subdomain ?? ""}.palform.app</strong
            > to sign in instead of https://dash.palform.app.
        </p>
        <p>
            If you need to deactivate OIDC and revert to password-based
            authentication, please get in touch with us.
        </p>
    </Alert>
{/if}

<form class="space-y-6" on:submit={onSubmit}>
    <Label>
        OIDC Discovery URL
        <Input
            class="mt-2"
            bind:value={discoveryUrl}
            type="url"
            disabled={loading}
            required
        />
        <Helper class="mt-2">
            An OpenID Discovery configuration file must be available at
            {discoveryUrl}/.well-known/openid-configuration
        </Helper>
    </Label>
    <Label>
        Client ID
        <Input class="mt-2" bind:value={clientId} disabled={loading} required />
        <Helper class="mt-2">
            Please configure the redirect URI https://{$orgCtx.org.subdomain ??
                ""}.palform.app/auth/callback for this client
        </Helper>
    </Label>
    <Label>
        Client secret
        <Input
            class="mt-2"
            bind:value={clientSecret}
            type={showSecret ? "text" : "password"}
            disabled={loading}
            required
        />
        <Helper class="mt-2">
            <TextButton on:click={onSecretToggle} disabled={loading}>
                {showSecret ? "Hide" : "Show"} secret
            </TextButton>
        </Helper>
    </Label>
    <Label>
        Team mapping field name
        <Input class="mt-2" bind:value={teamFieldName} disabled={loading} />
        <Helper class="mt-2">
            The name of a custom array-of-strings field your OIDC Provider will
            include in the ID Token, containing the names of teams to add the
            user to.
        </Helper>
        <Helper>
            Name-to-team mapping can be configured using the link below this
            form.
        </Helper>
        <Helper>
            This is optional; leave blank to not perform automatic team
            assignment.
        </Helper>
    </Label>
    <Toggle bind:checked={revokeTeamMappings}>
        Revoke team memberships when field no longer contains them
    </Toggle>

    <LoadingButton {loading} disabled={loading} type="submit">
        Save
    </LoadingButton>

    <TextButton
        class="block"
        href={`/orgs/${$orgCtx.org.id}/settings/auth/mappings`}
        on:click={navigateEvent}
    >
        Configure team mappings
    </TextButton>
</form>
