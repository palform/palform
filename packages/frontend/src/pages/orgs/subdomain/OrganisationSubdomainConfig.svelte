<script lang="ts">
    import { Alert, Helper, Input, Label } from "flowbite-svelte";
    import { isEntitled } from "../../../data/billing/entitlement";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import SubdomainConfigIntro from "./SubdomainConfigIntro.svelte";
    import InfoText from "../../../components/type/InfoText.svelte";
    import LoadingButton from "../../../components/LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import SubdomainPreview from "../../../components/orgs/subdomain/SubdomainPreview.svelte";

    const entitled = isEntitled("subdomain");
    const orgCtx = getOrgContext();

    let subdomain = "";
    let loading = false;
    $: onCreateClick = async (e: Event) => {
        e.preventDefault();
        loading = true;

        try {
            await APIs.orgs().then((a) =>
                a.orgsCreateSubdomain($orgCtx.org.id, {
                    subdomain,
                })
            );
            await showSuccessToast("Subdomain created");
            $orgCtx.org.subdomain = subdomain;
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

{#if !$entitled}
    <SubdomainConfigIntro />
{:else if !$orgCtx.org.subdomain}
    <Alert border class="mb-8">
        <h2 class="text-lg">Set up a subdomain</h2>
        <p>
            You'll be able to generate shortened links for your forms based on
            your subdomain, putting your brand front and centre.
        </p>
        <p>
            Currently, <strong>your subdomain cannot be changed</strong> after creation.
        </p>
    </Alert>

    <form on:submit={onCreateClick}>
        <Label>
            Choose your subdomain
            <Input class="mt-2" bind:value={subdomain} disabled={loading} />
            <Helper class="mt-2">
                Must be at least 3 characters. Certain reserved keywords are
                excluded. Subject to availability.
            </Helper>
        </Label>

        {#if subdomain}
            <InfoText class="mt-4">
                Your forms will be hosted on
                <SubdomainPreview class="mt-2" {subdomain} />
            </InfoText>

            <LoadingButton
                buttonClass="mt-4"
                type="submit"
                disabled={loading}
                {loading}
            >
                Create
            </LoadingButton>
        {/if}
    </form>
{:else}
    <InfoText>Your organisation has a subdomain set up:</InfoText>
    <InfoText>
        <SubdomainPreview subdomain={$orgCtx.org.subdomain} />
    </InfoText>

    <Alert class="mt-4">
        You can create short links for forms on this subdomain, and your users
        can sign into Palform on it if you have OIDC configured.
    </Alert>

    <Alert class="mt-4" color="yellow">
        Currently, we don't support deleting or modifying your subdomain. If you
        really need to change it, please get in touch.
    </Alert>
{/if}
