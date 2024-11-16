<script lang="ts">
    import {
        getRoundingAmountForBrand,
        type BrandContext,
    } from "../../../data/contexts/brand";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getTeamCtx } from "../../../data/contexts/team";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import FormFillFooter from "../../forms/fill/FormFillFooter.svelte";
    import QuestionFill from "../../questions/fill/QuestionFill.svelte";
    import ImageAsset from "../assets/ImageAsset.svelte";
    import BrandedButton from "./BrandedButton.svelte";
    import BrandingBackgroundColor from "./BrandingBackgroundColor.svelte";
    import BrandingContextProvider from "./BrandingContextProvider.svelte";
    import BrandingE2EeBadge from "./BrandingE2EEBadge.svelte";

    export let ctx: BrandContext;
    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();
</script>

<BrandingContextProvider {ctx}>
    <BrandingBackgroundColor />

    {#if ctx.background_image_asset_id}
        <div
            class="absolute w-full h-full brightness-50 flex items-center justify-center overflow-hidden bg-white dark:bg-slate-900"
        >
            <ImageAsset
                id={ctx.background_image_asset_id}
                teamId={$teamCtx.team.id}
                orgId={$orgCtx.org.id}
            />
        </div>
    {/if}

    <div class="py-10 px-[20%] absolute w-full h-full overflow-y-auto">
        <div
            class="space-y-6"
            style:border-radius={getRoundingAmountForBrand(ctx)}
        >
            {#if ctx.logo_asset_id}
                <ImageAsset
                    id={ctx.logo_asset_id}
                    teamId={$teamCtx.team.id}
                    orgId={$orgCtx.org.id}
                    width="140px"
                />
            {/if}

            {#if ctx.e2ee_badge}
                <BrandingE2EeBadge />
            {/if}

            <MainTitle>2024 Q2 Customer Survey</MainTitle>

            <QuestionFill
                isSample
                question={{
                    id: "a",
                    title: "What is your name?",
                    description: "Please enter your full, legal name",
                    required: true,
                    group_id: "a",
                    configuration: {
                        text: {
                            is_long: false,
                            validator: null,
                        },
                    },
                }}
            />
            <QuestionFill
                isSample
                question={{
                    id: "a",
                    title: "What's your favourite color?",
                    required: false,
                    group_id: "a",
                    configuration: {
                        choice: {
                            options: ["Purple", "Red", "Green", "Blue"],
                            multi: false,
                        },
                    },
                }}
            />
            <QuestionFill
                isSample
                question={{
                    id: "a",
                    title: "How would you rate your visit?",
                    required: false,
                    group_id: "a",
                    configuration: {
                        scale: {
                            min: 0,
                            min_label: "Awful",
                            max: 10,
                            max_label: "Amazing!",
                            icon: "Numeric",
                        },
                    },
                }}
            />

            <BrandedButton>Submit</BrandedButton>
            <BrandedButton outline>Clear</BrandedButton>

            <FormFillFooter dummyLinks />
        </div>
    </div>
</BrandingContextProvider>
