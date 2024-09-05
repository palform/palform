<script lang="ts">
    import Main from "../../layouts/Main.svelte";
    import MainTitle from "../../layouts/MainTitle.svelte";
    import {
        loadFormFill,
        fillSendStore,
        formFillStore,
        deleteFormFill,
        loadFormFillFromShortLink,
    } from "../../data/contexts/fill";
    import { Alert, Spinner } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faExclamationCircle } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import { humaniseAPIError } from "../../data/common";
    import QuestionGroupFiller from "../../components/questionGroups/fill/QuestionGroupFiller.svelte";
    import BrandingContextProvider from "../../components/teams/brandings/BrandingContextProvider.svelte";
    import ImageAsset from "../../components/teams/assets/ImageAsset.svelte";
    import { getOrgSubdomain } from "../../data/auth";
    import FormFillEndScreen from "../../components/forms/fill/FormFillEndScreen.svelte";
    import FormFillFooter from "../../components/forms/fill/FormFillFooter.svelte";
    import BrandingE2EeBadge from "../../components/teams/brandings/BrandingE2EEBadge.svelte";

    export let orgId: string | undefined;
    export let formId: string | undefined;
    export let fillShortLink: string | undefined;
    const fillAccessToken = new URLSearchParams(location.search).get("f");

    let initLoading = true;
    let initError: string | undefined = undefined;
    const loadInit = async () => {
        initLoading = true;
        initError = undefined;
        try {
            if (fillShortLink) {
                const subdomain = getOrgSubdomain();
                if (!subdomain) {
                    initError = "Organisation not found (missing subdomain)";
                    initLoading = false;
                    return;
                }

                await loadFormFillFromShortLink(subdomain, fillShortLink);
            } else if (fillAccessToken && orgId && formId) {
                await loadFormFill(orgId, formId, fillAccessToken);
            }
        } catch (e) {
            initError = humaniseAPIError(e, "That form");
        }
        initLoading = false;
    };
    onMount(() => loadInit());

    const newSubmission = async () => {
        $fillSendStore = undefined;
        await deleteFormFill();
        await loadInit();
    };
</script>

<BrandingContextProvider ctx={$formFillStore?.form.b}>
    {#if $formFillStore?.form.b?.background_image_asset_id}
        <ImageAsset
            id={$formFillStore.form.b.background_image_asset_id}
            asBodyBackground
        />
    {/if}
    {#if $formFillStore?.form.b?.e2ee_badge || !$formFillStore?.form.b}
        <BrandingE2EeBadge />
    {/if}
    <Main
        extraTight
        fullHeight
        verticalCenter={$formFillStore?.form.f.one_question_per_page}
    >
        {#if initLoading}
            <div class="text-center mb-10">
                <Spinner size={14} />
            </div>
        {:else if initError}
            <ErrorMsg
                e={initError}
                retryable
                on:retry={loadInit}
                targetDescriptor="form"
            />
        {:else if $formFillStore}
            {#if $formFillStore.form.b?.logo_asset_id}
                <ImageAsset
                    id={$formFillStore.form.b.logo_asset_id}
                    width="140px"
                    class="mb-8"
                    alt="Organisation logo"
                />
            {/if}

            {#if !$formFillStore.form.f.one_question_per_page}
                <MainTitle className="mb-8">
                    {$formFillStore.form.f.title}
                </MainTitle>
            {/if}

            {#if $fillSendStore?.done}
                <FormFillEndScreen on:restart={newSubmission} />
            {:else}
                <QuestionGroupFiller />
                {#if $fillSendStore?.error}
                    <Alert color="red" border class="mt-6">
                        <span slot="icon">
                            <FontAwesomeIcon icon={faExclamationCircle} />
                        </span>
                        {$fillSendStore.error}
                    </Alert>
                {/if}
            {/if}
        {/if}

        <FormFillFooter
            class={$formFillStore?.form.f.one_question_per_page
                ? "fixed bottom-4 left-4 md:w-full w-40"
                : "mt-6"}
        />
    </Main>
</BrandingContextProvider>
