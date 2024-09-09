<script lang="ts">
    import {
        faUndo,
        faUpRightFromSquare,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Alert } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import BrandedButton from "../../teams/brandings/BrandedButton.svelte";
    import { formFillStore } from "../../../data/contexts/fill";
    import MarkdownView from "../../markdown/MarkdownView.svelte";
    import BrandedSpan from "../../teams/brandings/BrandedSpan.svelte";
    import { t } from "../../../data/contexts/i18n";

    const dispatch = createEventDispatcher<{ restart: undefined }>();
    const endConfiguration = $formFillStore?.form.f.end_configuration;

    let redirecting = false;
    const onContinueClick = () => {
        if (!endConfiguration?.redirect_to) return;
        redirecting = true;
        window.location.href = endConfiguration.redirect_to;
    };
</script>

{#if endConfiguration}
    <Alert color="green" border>
        {#if endConfiguration.message}
            <p class="text-lg">
                <BrandedSpan>
                    <MarkdownView
                        value={endConfiguration.message}
                        imagesWithFillToken
                    />
                </BrandedSpan>
            </p>
        {/if}

        {#if $formFillStore.form.b === undefined || $formFillStore.form.b?.include_palform_attribution}
            <div class="mt-4 mb-2 text-gray-600 text-sm">
                <p>
                    <a
                        href="https://palform.app"
                        target="_blank"
                        class="hover:underline text-primary-700"
                    >
                        {t("form_end_attribution")}
                        <FontAwesomeIcon icon={faUpRightFromSquare} />
                    </a>
                </p>
            </div>
        {/if}

        <div class="flex items-center gap-3">
            {#if endConfiguration.redirect_to}
                <BrandedButton
                    class="mt-2"
                    on:click={onContinueClick}
                    disabled={redirecting}
                >
                    {t("form_end_continue")}
                </BrandedButton>
            {/if}
            {#if endConfiguration.show_restart}
                <BrandedButton
                    class="mt-2"
                    outline
                    on:click={() => dispatch("restart")}
                    disabled={redirecting}
                >
                    <FontAwesomeIcon icon={faUndo} class="me-2" />
                    {t("form_end_restart")}
                </BrandedButton>
            {/if}
        </div>
    </Alert>
{/if}
