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
    import { Alert } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faExclamationCircle } from "@fortawesome/free-solid-svg-icons";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import { humaniseAPIError } from "../../data/common";
    import QuestionGroupFiller from "../../components/questionGroups/fill/QuestionGroupFiller.svelte";
    import BrandingContextProvider from "../../components/teams/brandings/BrandingContextProvider.svelte";
    import ImageAsset from "../../components/teams/assets/ImageAsset.svelte";
    import { getOrgSubdomain } from "../../data/auth";
    import FormFillEndScreen from "../../components/forms/fill/FormFillEndScreen.svelte";
    import FormFillFooter from "../../components/forms/fill/FormFillFooter.svelte";
    import BrandingE2EeBadge from "../../components/teams/brandings/BrandingE2EEBadge.svelte";
    import FormFillLoading from "../../components/forms/fill/FormFillLoading.svelte";
    import BrandingBackgroundColor from "../../components/teams/brandings/BrandingBackgroundColor.svelte";
    import { isActive, route } from "../../router";

    let pathVars = $derived.by(() => {
        if (isActive("/fill/:orgId/:formId")) {
            return route.getParams("/fill/:orgId/:formId");
        } else if (isActive("/:fillShortLink")) {
            return route.getParams("/:fillShortLink");
        }

        throw new Error("Unrecognised path, cannot extract path vars");
    });
    const fillAccessToken = new URLSearchParams(location.search).get("f");

    let initLoading = $state(true);
    let initError: string | undefined = $state(undefined);
    const loadInit = async () => {
        initLoading = true;
        initError = undefined;
        try {
            if ("fillShortLink" in pathVars) {
                const subdomain = getOrgSubdomain();
                if (!subdomain) {
                    initError = "Organisation not found (missing subdomain)";
                    initLoading = false;
                    return;
                }

                await loadFormFillFromShortLink(
                    subdomain,
                    pathVars.fillShortLink
                );
            } else if (fillAccessToken && "orgId" in pathVars) {
                await loadFormFill(
                    pathVars.orgId,
                    pathVars.formId,
                    fillAccessToken,
                    false
                );
            }
        } catch (e) {
            initError = humaniseAPIError(e, "That form");
        }
        initLoading = false;
    };

    $effect(() => {
        pathVars;
        loadInit();
    });

    const newSubmission = async () => {
        $fillSendStore = undefined;
        await deleteFormFill();
        await loadInit();
    };

    let isFirstPage = $derived(
        $formFillStore &&
            $formFillStore.form.g.length > 0 &&
            $formFillStore.currentGroupId === $formFillStore.form.g[0].id
    );
</script>

{#if initLoading}
    <FormFillLoading />
{/if}

<BrandingContextProvider ctx={$formFillStore?.form.b}>
    {#if $formFillStore?.form.b?.background_image_asset_id}
        <ImageAsset
            id={$formFillStore.form.b.background_image_asset_id}
            asBodyBackground
        />
    {/if}

    <BrandingBackgroundColor />

    {#if $formFillStore?.form.b?.e2ee_badge || !$formFillStore?.form.b}
        <BrandingE2EeBadge />
    {/if}

    <Main
        extraTight
        fullHeight
        verticalCenter={$formFillStore?.form.f.one_question_per_page}
    >
        {#if initError}
            <ErrorMsg
                e={initError}
                retryable
                onretry={loadInit}
                targetDescriptor="form"
            />
        {:else if $formFillStore}
            {#if $formFillStore.form.b?.logo_asset_id && isFirstPage}
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
                        {#snippet icon()}
                            <span>
                                <FontAwesomeIcon icon={faExclamationCircle} />
                            </span>
                        {/snippet}
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
