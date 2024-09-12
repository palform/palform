import type {
    APIFillToken,
    APIQuestion,
    APIQuestionGroup,
    APIQuestionGroupStepStrategy,
    APIQuestionGroupStepStrategyOneOf,
} from "@paltiverse/palform-typescript-openapi";
import {
    submissionIsSuccess,
    type DecryptedSubmission,
    type DecryptedSubmissionSuccess,
} from "../crypto/results";
import { derived, readable, type Writable } from "svelte/store";
import { getContext, setContext } from "svelte";

export interface FormAdminContext {
    formId: string;
    questions: APIQuestion[];
    groups: APIQuestionGroup[];
    submissions: DecryptedSubmission[];
    tokens: APIFillToken[];
}

export function setFormAdminContext(ctx: Writable<FormAdminContext>) {
    // @ts-expect-error
    ctx.set({});
    setContext("responseCtx", ctx);
}
export function getFormAdminContext() {
    return getContext<Writable<FormAdminContext>>("responseCtx");
}
export function ctxGetGroup(groupId: string) {
    return derived(
        [readable(groupId), getFormAdminContext()],
        ([groupId, ctx]) => {
            return ctx.groups.find((e) => e.id === groupId);
        }
    );
}

export function ctxGetQuestion(questionId: string) {
    return derived(
        [readable(questionId), getFormAdminContext()],
        ([_, $ctx]) => {
            return $ctx.questions.find((q) => q.id === questionId);
        }
    );
}
export function ctxSubmissionsForQuestion(questionId: string) {
    return derived(
        [readable(questionId), getFormAdminContext()],
        ([_, $ctx]) => {
            const successfulOnly = $ctx.submissions.filter((s) =>
                submissionIsSuccess(s)
            ) as DecryptedSubmissionSuccess[];
            return successfulOnly.flatMap((s) =>
                s.questions.filter((q) => q.question_id === questionId)
            );
        }
    );
}

export function qgsIsJump(
    t: APIQuestionGroupStepStrategy
): t is APIQuestionGroupStepStrategyOneOf {
    return !(typeof t === "string");
}

export function getGroupTitle(
    oqpp: boolean,
    formCtx: FormAdminContext,
    group: APIQuestionGroup
) {
    if (oqpp) {
        const q = formCtx.questions.find((e) => e.group_id === group.id);
        if (!q) return "Untitled";
        return q.title;
    } else {
        if (!group.title) {
            return `Section TODO`;
        }
        return group.title;
    }
}
