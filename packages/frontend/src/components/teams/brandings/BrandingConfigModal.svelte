<script lang="ts">
    import {
        Button,
        ButtonGroup,
        Helper,
        Input,
        Label,
        Toggle,
    } from "flowbite-svelte";
    import GoogleFontPicker from "./GoogleFontPicker.svelte";
    import type {
        APIFormBranding,
        APIFormBrandingRequest,
        FormBrandingBorderRoundingEnum,
        FormBrandingFontSizeEnum,
        FormBrandingSpacingEnum,
    } from "@palform/palform-typescript-openapi";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getTeamCtx } from "../../../data/contexts/team";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import TeamAssetInput from "../assets/TeamAssetInput.svelte";
    import { scale } from "svelte/transition";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faFont,
        faPlus,
        faTimes,
        faTrash,
    } from "@fortawesome/free-solid-svg-icons";
    import SectionSeparator from "../../type/SectionSeparator.svelte";
    import BrandingColorPicker from "./BrandingColorPicker.svelte";
    import BrandingConfigPreview from "./BrandingConfigPreview.svelte";
    import RadioButtonGroup from "../../radioButton/RadioButtonGroup.svelte";

    interface Props {
        modalOpen: boolean;
        existingBranding?: APIFormBranding | undefined;
    }

    let { modalOpen = $bindable(), existingBranding = undefined }: Props =
        $props();
    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();

    let primaryColor = $state(existingBranding?.primary_color ?? "#3584e4");
    let accentColor = $state(existingBranding?.accent_color ?? "#33d17a");
    let fontFamily = $state(existingBranding?.google_font ?? "Roboto");
    let brandingName = $state(existingBranding?.name ?? "My custom scheme");
    let borderRounding: FormBrandingBorderRoundingEnum = $state(
        existingBranding?.border_rounding ?? "Medium"
    );
    let spacing: FormBrandingSpacingEnum = $state(
        existingBranding?.spacing ?? "Normal"
    );
    let fontSize: FormBrandingFontSizeEnum = $state(
        existingBranding?.font_size ?? "Regular"
    );
    let logoId: string | null = $state(existingBranding?.logo_asset_id ?? null);
    let backgroundId: string | null = $state(
        existingBranding?.background_image_asset_id ?? null
    );
    let includeAttribution = $state(
        existingBranding?.include_palform_attribution ?? false
    );
    let extraFooterMessage = $state(
        existingBranding?.extra_footer_message ?? null
    );
    let termsLink = $state(existingBranding?.terms_link ?? null);
    let privacyLink = $state(existingBranding?.privacy_link ?? null);
    let borderIntensity = $state(
        existingBranding?.border_intensity ?? "Medium"
    );
    let borderShadowIntensity = $state(
        existingBranding?.border_shadow_intensity ?? "Medium"
    );
    let e2eeBadge = $state(existingBranding?.e2ee_badge ?? true);
    let backgroundColor = $state(
        existingBranding?.background_color ?? undefined
    );
    let backgroundColorAccent = $state(
        existingBranding?.background_color_accent ?? undefined
    );

    let loading = $state(false);
    let onSaveClick = $derived(async () => {
        if (brandingName === "") {
            await showFailureToast("Please set a nickname");
            return;
        }

        loading = true;
        const api = await APIs.formBrandings();

        const branding: APIFormBrandingRequest = {
            name: brandingName,
            google_font: fontFamily,
            primary_color: primaryColor,
            accent_color: accentColor,
            border_rounding: borderRounding,
            spacing: spacing,
            font_size: fontSize,
            logo_asset_id: logoId,
            background_image_asset_id: backgroundId,
            include_palform_attribution: includeAttribution,
            extra_footer_message: extraFooterMessage,
            terms_link: termsLink,
            privacy_link: privacyLink,
            border_intensity: borderIntensity,
            border_shadow_intensity: borderShadowIntensity,
            e2ee_badge: e2eeBadge,
            background_color: backgroundColor,
            background_color_accent: backgroundColorAccent,
        };

        try {
            if (existingBranding) {
                await api.organisationTeamBrandingPut(
                    $orgCtx.org.id,
                    $teamCtx.team.id,
                    existingBranding.id,
                    branding
                );
                teamCtx.update((ctx) => {
                    const i = ctx.brandings.findIndex(
                        (e) => e.id === existingBranding.id
                    );
                    if (i === -1) return ctx;
                    ctx.brandings[i] = {
                        ...ctx.brandings[i],
                        ...branding,
                    };
                    return ctx;
                });
            } else {
                const resp = await api.organisationTeamBrandingCreate(
                    $orgCtx.org.id,
                    $teamCtx.team.id,
                    branding
                );
                teamCtx.update((ctx) => {
                    return {
                        ...ctx,
                        brandings: [
                            ...ctx.brandings,
                            {
                                ...branding,
                                team_id: $teamCtx.team.id,
                                id: resp.data,
                            },
                        ],
                    };
                });
            }
            modalOpen = false;
            await showSuccessToast("Branding scheme saved");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

{#if modalOpen}
    <div
        class="fixed top-0 left-0 w-screen h-screen max-h-screen bg-white dark:bg-slate-900 flex flex-col overflow-hidden"
        transition:scale
    >
        <div
            class="h-16 border-b border-b-slate-300 dark:border-b-slate-600 flex items-center justify-between px-10 gap-10"
        >
            <h2
                class="text-xl text-gray-700 dark:text-gray-300 font-display font-medium"
            >
                Configure branding scheme
            </h2>
            <Button
                outline
                class="px-3! py-2!"
                onclick={() => (modalOpen = false)}
            >
                <FontAwesomeIcon icon={faTimes} size="lg" />
            </Button>
        </div>

        <div class="flex-auto flex overflow-y-hidden">
            <div
                class="w-2/6 p-4 border-r dark:border-r-slate-600 space-y-4 overflow-y-auto"
            >
                <Label>
                    Primary color
                    <BrandingColorPicker
                        bind:value={primaryColor}
                        disabled={loading}
                        name="primary_color"
                    />
                </Label>
                <Label>
                    Accent color
                    <BrandingColorPicker
                        bind:value={accentColor}
                        disabled={loading}
                        name="accent_color"
                    />
                </Label>

                <Label>
                    Font family
                    <GoogleFontPicker
                        class="mt-1"
                        bind:selectedFont={fontFamily}
                        disabled={loading}
                    />
                    <Helper class="mt-2">
                        Fonts from <a
                            href="https://fonts.google.com"
                            target="_blank"
                            rel="noreferer"
                            class="underline">Google Fonts</a
                        >. Need a custom font? Please get in touch!
                    </Helper>
                </Label>

                <div>
                    <Label>Font size</Label>
                    <RadioButtonGroup
                        class="mt-2"
                        bind:selectedValue={fontSize}
                        conjoined
                        values={[
                            {
                                value: "Tiny",
                                icon: faFont,
                                iconSize: "xs",
                                tooltip: "Tiny",
                            },
                            {
                                value: "Small",
                                icon: faFont,
                                iconSize: "sm",
                                tooltip: "Small",
                            },
                            {
                                value: "Regular",
                                icon: faFont,
                                tooltip: "Regular",
                            },
                            {
                                value: "Large",
                                icon: faFont,
                                iconSize: "lg",
                                tooltip: "Large",
                            },
                            {
                                value: "VeryLarge",
                                icon: faFont,
                                iconSize: "xl",
                                tooltip: "Very large",
                            },
                        ]}
                    />
                </div>

                <SectionSeparator />

                <div>
                    <Label>Border rounding</Label>
                    <RadioButtonGroup
                        class="mt-2"
                        bind:selectedValue={borderRounding}
                        values={[
                            { value: "None", label: "None" },
                            { value: "Small", label: "Small" },
                            { value: "Medium", label: "Medium" },
                            { value: "Large", label: "Large" },
                        ]}
                    />
                </div>

                <div>
                    <Label>Question border intensity</Label>
                    <RadioButtonGroup
                        class="mt-2"
                        bind:selectedValue={borderIntensity}
                        values={[
                            { value: "Off", label: "No border" },
                            { value: "Low", label: "Low" },
                            { value: "Medium", label: "Medium" },
                            { value: "High", label: "High" },
                        ]}
                    />
                </div>

                <div>
                    <Label>Question border shadow intensity</Label>
                    <RadioButtonGroup
                        class="mt-2"
                        bind:selectedValue={borderShadowIntensity}
                        values={[
                            { value: "Off", label: "No shadow" },
                            { value: "Low", label: "Low" },
                            { value: "Medium", label: "Medium" },
                            { value: "High", label: "High" },
                        ]}
                    />
                </div>

                <SectionSeparator />

                <div>
                    <Label>Spacing</Label>
                    <RadioButtonGroup
                        class="mt-2"
                        bind:selectedValue={spacing}
                        values={[
                            { value: "Tight", label: "Tight" },
                            { value: "Normal", label: "Normal" },
                            { value: "Comfy", label: "Comfy" },
                        ]}
                    />
                </div>

                <SectionSeparator />

                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <Label for="logoID">Logo to show at top</Label>
                        <TeamAssetInput
                            id="logoID"
                            bind:value={logoId}
                            teamId={$teamCtx.team.id}
                            class="mt-1"
                        />
                    </div>
                    <div>
                        <Label for="bgID">Background image</Label>
                        <TeamAssetInput
                            id="bgID"
                            bind:value={backgroundId}
                            teamId={$teamCtx.team.id}
                            class="mt-1"
                        />
                    </div>
                </div>

                <Label>
                    Background color
                    <BrandingColorPicker
                        bind:value={backgroundColor}
                        disabled={loading}
                        name="background_color"
                        includeNullOption
                        pastel
                    />
                </Label>
                <Label>
                    Background accent
                    <BrandingColorPicker
                        bind:value={backgroundColorAccent}
                        disabled={loading}
                        name="background_color_accent"
                        includeNullOption
                        pastel
                    />
                </Label>

                <SectionSeparator />

                <Toggle bind:checked={includeAttribution}>
                    Include Palform attribution in footer
                </Toggle>

                <Toggle bind:checked={e2eeBadge}>
                    Show end-to-end encryption badge
                </Toggle>

                {#if termsLink === null}
                    <Button
                        size="xs"
                        color="light"
                        onclick={() => (termsLink = "")}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add terms link
                    </Button>
                {:else}
                    <Label>
                        Terms of Use link
                        <ButtonGroup class="flex mt-1">
                            <Input bind:value={termsLink} />
                            <Button onclick={() => (termsLink = null)}>
                                <FontAwesomeIcon icon={faTrash} />
                            </Button>
                        </ButtonGroup>
                    </Label>
                {/if}
                {#if privacyLink === null}
                    <Button
                        size="xs"
                        color="light"
                        onclick={() => (privacyLink = "")}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add privacy link
                    </Button>
                {:else}
                    <Label>
                        Privacy Policy link
                        <ButtonGroup class="flex mt-1">
                            <Input bind:value={privacyLink} />
                            <Button onclick={() => (privacyLink = null)}>
                                <FontAwesomeIcon icon={faTrash} />
                            </Button>
                        </ButtonGroup>
                    </Label>
                {/if}
                {#if extraFooterMessage === null}
                    <Button
                        size="xs"
                        color="light"
                        onclick={() => (extraFooterMessage = "")}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add extra message to footer
                    </Button>
                {:else}
                    <Label>
                        Extra footer message
                        <ButtonGroup class="flex mt-1">
                            <Input bind:value={extraFooterMessage} />
                            <Button onclick={() => (extraFooterMessage = null)}>
                                <FontAwesomeIcon icon={faTrash} />
                            </Button>
                        </ButtonGroup>
                    </Label>
                {/if}

                <SectionSeparator />

                <Label>
                    Scheme nickname
                    <Input
                        bind:value={brandingName}
                        required
                        class="mt-1"
                        disabled={loading}
                    />
                    <Helper class="mt-1">
                        Create a helpful nickname to find your branding scheme
                        when creating forms
                    </Helper>
                </Label>

                <LoadingButton
                    disabled={loading}
                    {loading}
                    onclick={onSaveClick}
                >
                    Save
                </LoadingButton>
            </div>
            <div class="flex-1 relative">
                <BrandingConfigPreview
                    ctx={{
                        primary_color: primaryColor,
                        accent_color: accentColor,
                        google_font: fontFamily,
                        font_size: fontSize,
                        border_rounding: borderRounding,
                        spacing: spacing,
                        logo_asset_id: logoId,
                        background_image_asset_id: backgroundId,
                        terms_link: termsLink,
                        privacy_link: privacyLink,
                        extra_footer_message: extraFooterMessage,
                        include_palform_attribution: includeAttribution,
                        border_intensity: borderIntensity,
                        border_shadow_intensity: borderShadowIntensity,
                        e2ee_badge: e2eeBadge,
                        background_color: backgroundColor,
                        background_color_accent: backgroundColorAccent,
                    }}
                />
            </div>
        </div>
    </div>
{/if}
