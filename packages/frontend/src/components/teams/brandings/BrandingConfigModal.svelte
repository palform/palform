<script lang="ts">
    import {
        Button,
        ButtonGroup,
        Helper,
        Input,
        Label,
        RadioButton,
        Toggle,
        Tooltip,
    } from "flowbite-svelte";
    import GoogleFontPicker from "./GoogleFontPicker.svelte";
    import type {
        APIFormBranding,
        APIFormBrandingRequest,
        FormBrandingBorderRoundingEnum,
        FormBrandingFontSizeEnum,
        FormBrandingSpacingEnum,
    } from "@paltiverse/palform-typescript-openapi";
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

    export let modalOpen: boolean;
    export let existingBranding: APIFormBranding | undefined = undefined;
    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();

    let primaryColor = existingBranding?.primary_color ?? "#3584e4";
    let accentColor = existingBranding?.accent_color ?? "#33d17a";
    let fontFamily = existingBranding?.google_font ?? "Roboto";
    let brandingName = existingBranding?.name ?? "My custom scheme";
    let borderRounding: FormBrandingBorderRoundingEnum =
        existingBranding?.border_rounding ?? "Medium";
    let spacing: FormBrandingSpacingEnum =
        existingBranding?.spacing ?? "Normal";
    let fontSize: FormBrandingFontSizeEnum =
        existingBranding?.font_size ?? "Regular";
    let logoId: string | null = existingBranding?.logo_asset_id ?? null;
    let backgroundId: string | null =
        existingBranding?.background_image_asset_id ?? null;
    let includeAttribution =
        existingBranding?.include_palform_attribution ?? false;
    let extraFooterMessage = existingBranding?.extra_footer_message ?? null;
    let termsLink = existingBranding?.terms_link ?? null;
    let privacyLink = existingBranding?.privacy_link ?? null;
    let borderIntensity = existingBranding?.border_intensity ?? "Medium";
    let borderShadowIntensity =
        existingBranding?.border_shadow_intensity ?? "Medium";
    let e2eeBadge = existingBranding?.e2ee_badge ?? true;
    let backgroundColor = existingBranding?.background_color ?? null;
    let backgroundColorAccent =
        existingBranding?.background_color_accent ?? null;

    let loading = false;
    $: onSaveClick = async () => {
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
    };
</script>

{#if modalOpen}
    <div
        class="fixed top-0 left-0 w-screen h-screen max-h-screen bg-white dark:bg-slate-900 flex flex-col overflow-hidden"
        transition:scale
    >
        <div
            class="h-16 border-b dark:border-b-slate-600 flex items-center justify-between px-10 gap-10"
        >
            <h2
                class="text-xl text-gray-700 dark:text-gray-300 font-display font-medium"
            >
                Configure branding scheme
            </h2>
            <Button
                outline
                class="!px-3 !py-2"
                on:click={() => (modalOpen = false)}
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
                    <ButtonGroup class="mt-2">
                        <RadioButton value={"Tiny"} bind:group={fontSize}>
                            <FontAwesomeIcon icon={faFont} size="xs" />
                        </RadioButton>
                        <Tooltip>Small</Tooltip>
                        <RadioButton value={"Small"} bind:group={fontSize}>
                            <FontAwesomeIcon icon={faFont} size="sm" />
                        </RadioButton>
                        <Tooltip>Small</Tooltip>
                        <RadioButton value={"Regular"} bind:group={fontSize}>
                            <FontAwesomeIcon icon={faFont} />
                        </RadioButton>
                        <Tooltip>Regular</Tooltip>
                        <RadioButton value={"Large"} bind:group={fontSize}>
                            <FontAwesomeIcon icon={faFont} size="lg" />
                        </RadioButton>
                        <Tooltip>Large</Tooltip>
                        <RadioButton value={"VeryLarge"} bind:group={fontSize}>
                            <FontAwesomeIcon icon={faFont} size="xl" />
                        </RadioButton>
                        <Tooltip>Very large</Tooltip>
                    </ButtonGroup>
                </div>

                <SectionSeparator />

                <div>
                    <Label>Border rounding</Label>
                    <div class="flex gap-2 mt-2">
                        <RadioButton
                            value={"None"}
                            bind:group={borderRounding}
                            color="light"
                            class="!rounded-none"
                        >
                            None
                        </RadioButton>
                        <RadioButton
                            value={"Small"}
                            bind:group={borderRounding}
                            color="light"
                            class="!rounded-md"
                        >
                            Small
                        </RadioButton>
                        <RadioButton
                            value={"Medium"}
                            bind:group={borderRounding}
                            color="light"
                            class="!rounded-xl"
                        >
                            Medium
                        </RadioButton>
                        <RadioButton
                            value={"Large"}
                            bind:group={borderRounding}
                            color="light"
                            class="!rounded-3xl"
                        >
                            Large
                        </RadioButton>
                    </div>
                </div>

                <div>
                    <Label>Question border intensity</Label>
                    <div class="flex gap-2 mt-2">
                        <RadioButton
                            value={"Off"}
                            bind:group={borderIntensity}
                            color="light"
                        >
                            No border
                        </RadioButton>
                        <RadioButton
                            value={"Low"}
                            bind:group={borderIntensity}
                            color="light"
                        >
                            Low
                        </RadioButton>
                        <RadioButton
                            value={"Medium"}
                            bind:group={borderIntensity}
                            color="light"
                        >
                            Medium
                        </RadioButton>
                        <RadioButton
                            value={"High"}
                            bind:group={borderIntensity}
                            color="light"
                        >
                            High
                        </RadioButton>
                    </div>
                </div>

                <div>
                    <Label>Question border shadow intensity</Label>
                    <div class="flex gap-2 mt-2">
                        <RadioButton
                            value={"Off"}
                            bind:group={borderShadowIntensity}
                            color="light"
                        >
                            No shadow
                        </RadioButton>
                        <RadioButton
                            value={"Low"}
                            bind:group={borderShadowIntensity}
                            color="light"
                        >
                            Low
                        </RadioButton>
                        <RadioButton
                            value={"Medium"}
                            bind:group={borderShadowIntensity}
                            color="light"
                        >
                            Medium
                        </RadioButton>
                        <RadioButton
                            value={"High"}
                            bind:group={borderShadowIntensity}
                            color="light"
                        >
                            High
                        </RadioButton>
                    </div>
                </div>

                <SectionSeparator />

                <div>
                    <Label>Spacing</Label>
                    <div class="flex gap-2 mt-2">
                        <RadioButton
                            value={"Tight"}
                            bind:group={spacing}
                            color="light"
                            class="!px-2"
                        >
                            Tight
                        </RadioButton>
                        <RadioButton
                            value={"Normal"}
                            bind:group={spacing}
                            color="light"
                        >
                            Normal
                        </RadioButton>
                        <RadioButton
                            value={"Comfy"}
                            bind:group={spacing}
                            color="light"
                            class="!px-7"
                        >
                            Comfy
                        </RadioButton>
                    </div>
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
                        on:click={() => (termsLink = "")}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add terms link
                    </Button>
                {:else}
                    <Label>
                        Terms of Use link
                        <ButtonGroup class="flex mt-1">
                            <Input bind:value={termsLink} />
                            <Button on:click={() => (termsLink = null)}>
                                <FontAwesomeIcon icon={faTrash} />
                            </Button>
                        </ButtonGroup>
                    </Label>
                {/if}
                {#if privacyLink === null}
                    <Button
                        size="xs"
                        color="light"
                        on:click={() => (privacyLink = "")}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add privacy link
                    </Button>
                {:else}
                    <Label>
                        Privacy Policy link
                        <ButtonGroup class="flex mt-1">
                            <Input bind:value={privacyLink} />
                            <Button on:click={() => (privacyLink = null)}>
                                <FontAwesomeIcon icon={faTrash} />
                            </Button>
                        </ButtonGroup>
                    </Label>
                {/if}
                {#if extraFooterMessage === null}
                    <Button
                        size="xs"
                        color="light"
                        on:click={() => (extraFooterMessage = "")}
                    >
                        <FontAwesomeIcon icon={faPlus} class="me-2" />
                        Add extra message to footer
                    </Button>
                {:else}
                    <Label>
                        Extra footer message
                        <ButtonGroup class="flex mt-1">
                            <Input bind:value={extraFooterMessage} />
                            <Button
                                on:click={() => (extraFooterMessage = null)}
                            >
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
                    on:click={onSaveClick}
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
