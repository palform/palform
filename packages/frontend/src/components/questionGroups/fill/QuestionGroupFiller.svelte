<script lang="ts">
    import {
        ctxGetCurrentGroup,
        ctxGetCurrentGroupQuestions,
        ctxGetNextStep,
        fillSendStore,
        formFillStore,
        validateQuestions,
    } from "../../../data/contexts/fill";
    import QuestionFill from "../../questions/fill/QuestionFill.svelte";
    import QgFillHeader from "./QGFillHeader.svelte";
    import { sendSubmission } from "../../../data/crypto/submissions";
    import { humaniseAPIError } from "../../../data/common";
    import BrandedButton from "../../teams/brandings/BrandedButton.svelte";
    import FormFillCaptchaModal from "../../forms/fill/FormFillCaptchaModal.svelte";
    import { t } from "../../../data/contexts/i18n";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight, faCheck } from "@fortawesome/free-solid-svg-icons";

    const currentGroup = ctxGetCurrentGroup();
    const currentGroupQuestions = ctxGetCurrentGroupQuestions();
    const nextStep = ctxGetNextStep();

    let animateOut = true;
    const animationDelay = 100;
    $: $currentGroup?.id,
        setTimeout(() => {
            animateOut = false;
        }, animationDelay);

    let showCaptchaModal = false;
    $: onSubmit = async (e: Event, captchaValue?: string) => {
        e.preventDefault();

        if (!$formFillStore) return;
        if (!validateQuestions()) return;

        if (
            $formFillStore.form.f.enable_captcha &&
            captchaValue === undefined
        ) {
            showCaptchaModal = true;
            return;
        }

        if (captchaValue !== undefined) {
            showCaptchaModal = false;
        }

        $fillSendStore = {
            loading: true,
            error: undefined,
            done: false,
        };

        try {
            await sendSubmission(
                $formFillStore.submission,
                $formFillStore.organisationId,
                $formFillStore.form.f.id,
                $formFillStore.fillAccessToken,
                $formFillStore.currentGroupId,
                !$formFillStore.isShortLink,
                captchaValue
            );
        } catch (e) {
            $fillSendStore = {
                loading: false,
                error: humaniseAPIError(e),
                done: false,
            };
            return;
        }

        $fillSendStore = {
            loading: false,
            error: undefined,
            done: true,
        };
    };

    $: onNext = (e: Event) => {
        e.preventDefault();

        if ($formFillStore === undefined || $nextStep === undefined) return;
        if (!validateQuestions()) return;

        animateOut = true;
        setTimeout(() => {
            formFillStore.update((ctx) => {
                if (!ctx) return undefined;
                return {
                    ...ctx,
                    submission: {
                        ...ctx.submission,
                        groups_completed: [
                            ...ctx.submission.groups_completed,
                            ctx.currentGroupId,
                        ],
                    },
                    currentGroupId: $nextStep,
                };
            });
        }, animationDelay);
    };

    $: onPrevious = () => {
        if (
            $formFillStore === undefined ||
            $formFillStore.submission.groups_completed.length === 0
        )
            return;

        animateOut = true;
        setTimeout(() => {
            formFillStore.update((ctx) => {
                if (ctx === undefined) return undefined;
                const newStep = ctx.submission.groups_completed.pop();
                if (!newStep) return ctx;
                ctx.currentGroupId = newStep;
                return ctx;
            });
        }, animationDelay);
    };
</script>

<QgFillHeader class="mb-4" />

{#if $formFillStore !== undefined && $currentGroup !== undefined}
    <form
        class={`space-y-8 transition ${animateOut ? "translate-y-8 opacity-0 pointer-events-none" : ""}`}
        on:submit={(e) => ($nextStep === undefined ? onSubmit(e) : onNext(e))}
    >
        {#each $currentGroupQuestions as question (question.id)}
            <QuestionFill {question} />
        {/each}

        <div class="space-x-2">
            {#if $nextStep === undefined}
                <BrandedButton
                    disabled={$fillSendStore?.loading}
                    loading={$fillSendStore?.loading}
                    type="submit"
                >
                    {t("submit")}
                    <FontAwesomeIcon icon={faCheck} class="ms-2" />
                </BrandedButton>
            {:else}
                <BrandedButton type="submit">
                    {t("next")}
                    <FontAwesomeIcon icon={faArrowRight} class="ms-2" />
                </BrandedButton>
            {/if}
            {#if $formFillStore.submission.groups_completed.length > 0}
                <BrandedButton on:click={onPrevious} outline>
                    {t("back")}
                </BrandedButton>
            {/if}
        </div>
    </form>

    <FormFillCaptchaModal
        bind:open={showCaptchaModal}
        on:complete={(e) => onSubmit(e, e.detail)}
    />
{/if}
