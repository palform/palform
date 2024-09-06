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

    const currentGroup = ctxGetCurrentGroup();
    const currentGroupQuestions = ctxGetCurrentGroupQuestions();
    const nextStep = ctxGetNextStep();

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
    };

    $: onPrevious = () => {
        if (
            $formFillStore === undefined ||
            $formFillStore.submission.groups_completed.length === 0
        )
            return;

        formFillStore.update((ctx) => {
            if (ctx === undefined) return undefined;
            const newStep = ctx.submission.groups_completed.pop();
            if (!newStep) return ctx;
            ctx.currentGroupId = newStep;
            return ctx;
        });
    };
</script>

<QgFillHeader class="mb-4" />

{#if $formFillStore !== undefined && $currentGroup !== undefined}
    <form
        class="space-y-8"
        on:submit={(e) => ($nextStep === undefined ? onSubmit(e) : onNext(e))}
    >
        {#each $currentGroupQuestions as question (question.id)}
            <QuestionFill {question} />
        {/each}

        <div class="space-x-2 !mb-10">
            {#if $nextStep === undefined}
                <BrandedButton
                    disabled={$fillSendStore?.loading}
                    loading={$fillSendStore?.loading}
                    type="submit"
                >
                    Submit
                </BrandedButton>
            {:else}
                <BrandedButton type="submit">Next</BrandedButton>
            {/if}
            {#if $formFillStore.submission.groups_completed.length > 0}
                <BrandedButton on:click={onPrevious} outline>
                    Back
                </BrandedButton>
            {/if}
        </div>
    </form>

    <FormFillCaptchaModal
        bind:open={showCaptchaModal}
        on:complete={(e) => onSubmit(e, e.detail)}
    />
{/if}
