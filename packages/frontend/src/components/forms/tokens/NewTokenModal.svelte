<script lang="ts">
    import {
        Alert,
        Button,
        Helper,
        Input,
        Label,
        Modal,
        Select,
    } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import {
        copyFillToken,
        formatFillTokenURL,
        formatShortLinkURL,
    } from "../../../data/fillTokens";
    import { createEventDispatcher } from "svelte";
    import type { APIFillToken } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getResponsesContext } from "../../../data/contexts/results";
    import expiryTimeOptions from "../../../data/util/expiryTimeOptions";
    import { showFailureToast } from "../../../data/toast";
    import { isEntitled } from "../../../data/billing/entitlement";
    import { Link } from "svelte-routing";
    import { copyGenericValue } from "../../../data/util/clipboard";
    import TokenEmbedOptions from "./TokenEmbedOptions.svelte";

    const orgCtx = getOrgContext();
    const respCtx = getResponsesContext();
    const subdomainEntitled = isEntitled("subdomain");

    let modalOpen = false;
    let expiryValue: null | number = null;
    let nicknameValue = "Default";
    let shortLinkValue = "";
    let loading = false;
    let justCreated: string | undefined = undefined;

    const dispatch = createEventDispatcher<{ newToken: APIFillToken }>();

    $: onShareClick = async (e: SubmitEvent) => {
        e.preventDefault();
        loading = true;
        try {
            const resp = await APIs.fillTokens().then((a) =>
                a.fillAccessTokensCreate($orgCtx.org.id, $respCtx.formId, {
                    expires_in_seconds:
                        expiryValue === null ? null : expiryValue * 60,
                    nickname: nicknameValue,
                    short_link: shortLinkValue || null,
                })
            );
            justCreated = resp.data.id;
            dispatch("newToken", resp.data);
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };

    $: onURLClick = async () => {
        if (!justCreated) return;
        await copyFillToken($orgCtx.org.id, $respCtx.formId, justCreated);
    };
    $: onShortLinkCopy = async () => {
        if (!shortLinkValue || !$orgCtx.org.subdomain) return;
        await copyGenericValue(
            formatShortLinkURL($orgCtx.org.subdomain, shortLinkValue)
        );
    };

    const closeModal = () => {
        justCreated = undefined;
        modalOpen = false;
    };
    $: {
        if (modalOpen === false) justCreated = undefined;
    }
</script>

<Button class="mb-4" outline on:click={() => (modalOpen = true)}>
    <FontAwesomeIcon icon={faPlus} class="mr-2" />
    Publish form
</Button>

<Modal bind:open={modalOpen} title="Publish form" outsideclose>
    {#if justCreated}
        <Alert color="green">
            Yay! You've created a Share Token. Copy the URL below and send it to
            anyone you want to fill in your form.
        </Alert>

        <TokenEmbedOptions
            fatID={justCreated}
            shortLink={shortLinkValue || undefined}
        />

        <Button class="mt-6" on:click={closeModal}>Close</Button>
    {:else}
        <p>
            This will create a new Share Token (a unique URL) you can distribute
            to anyone who needs to fill your form.
        </p>

        <form on:submit={onShareClick} class="space-y-6">
            <Label>
                Nickname
                <Input
                    class="mt-2"
                    bind:value={nicknameValue}
                    disabled={loading}
                    required
                />
                <Helper class="mt-2">
                    Responses will be associated with the Share Token used to
                    submit them. You can use clear nicknames to help segment
                    respondents.
                </Helper>
            </Label>

            <Label>
                Expires in
                <Select
                    class="mt-2"
                    bind:value={expiryValue}
                    disabled={loading}
                    items={expiryTimeOptions(true)}
                    required
                />
            </Label>

            {#if $subdomainEntitled}
                {#if $orgCtx.org.subdomain}
                    <Label>
                        Short link (optional)
                        <Input
                            class="mt-2"
                            bind:value={shortLinkValue}
                            disabled={loading}
                        />
                        {#if shortLinkValue.length > 0}
                            <Helper class="mt-2">
                                Nice! Your form will be available at
                                {$orgCtx.org.subdomain}.palform.app/<strong
                                    >{shortLinkValue}</strong
                                >
                            </Helper>
                        {/if}
                    </Label>
                {:else}
                    <Alert color="blue">
                        Your subscription includes <strong>short links</strong>
                        for forms. To start using them, <Link
                            to={`/orgs/${$orgCtx.org.id}/settings/subdomain`}
                            class="underline">set up a subdomain</Link
                        >.
                    </Alert>
                {/if}
            {/if}

            <LoadingButton
                buttonClass="mt-6"
                {loading}
                disabled={loading}
                type="submit"
            >
                Share!
            </LoadingButton>
        </form>
    {/if}
</Modal>
