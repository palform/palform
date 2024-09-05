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
import { derived, get, readable, type Writable } from "svelte/store";
import { getContext, setContext } from "svelte";
import { APIs } from "../common";

export interface ResponsesContext {
    formId: string;
    questions: APIQuestion[];
    groups: APIQuestionGroup[];
    submissions: DecryptedSubmission[];
    tokens: APIFillToken[];
}

export function setResponsesContext(ctx: Writable<ResponsesContext>) {
    // @ts-expect-error
    ctx.set({});
    setContext("responseCtx", ctx);
}
export function getResponsesContext() {
    return getContext<Writable<ResponsesContext>>("responseCtx");
}
export function ctxGetGroup(groupId: string) {
    return derived(
        [readable(groupId), getResponsesContext()],
        ([groupId, ctx]) => {
            return ctx.groups.find((e) => e.id === groupId);
        }
    );
}
export function ctxUpdateGroup(
    ctx: Writable<ResponsesContext>,
    groupId: string,
    group: APIQuestionGroup
) {
    ctx.update((ctx) => {
        const i = ctx.groups.findIndex((e) => e.id === groupId);
        if (i === -1) return ctx;
        ctx.groups[i] = group;
        return ctx;
    });
}

export function ctxGetQuestion(questionId: string) {
    return derived(
        [readable(questionId), getResponsesContext()],
        ([_, $ctx]) => {
            return $ctx.questions.find((q) => q.id === questionId);
        }
    );
}
export function ctxSubmissionsForQuestion(questionId: string) {
    return derived(
        [readable(questionId), getResponsesContext()],
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

export async function saveGroupsWithPositions(
    ctx: Writable<ResponsesContext>,
    orgId: string
) {
    const val = get(ctx);
    const api = await APIs.questionGroups();
    for (let i = 0; i < val.groups.length; i++) {
        const g = val.groups[i];
        await api.questionGroupsSet(orgId, val.formId, g.id, {
            ...g,
            position: i,
        });
        ctx.update((c) => {
            c.groups[i].position = i;
            return c;
        });
    }
}
export async function insertGroup(
    ctx: Writable<ResponsesContext>,
    orgId: string,
    beforeIndex: number,
    title: string | null,
    description: string | null
) {
    const ctxVal = get(ctx);

    const obj = {
        position: beforeIndex,
        title,
        description,
    } as Omit<APIQuestionGroup, "id">;
    const resp = await APIs.questionGroups().then((a) =>
        a.questionGroupsCreate(orgId, ctxVal.formId, obj)
    );
    ctx.update((ctx) => {
        ctx.groups.splice(beforeIndex, 0, {
            ...obj,
            id: resp.data,
            step_strategy: "NextPosition",
        });
        return ctx;
    });
    await saveGroupsWithPositions(ctx, orgId);

    return resp.data;
}

export async function deleteGroup(
    ctx: Writable<ResponsesContext>,
    orgId: string,
    groupId: string
) {
    const ctxVal = get(ctx);
    await APIs.questionGroups().then((a) =>
        a.questionGroupsDelete(orgId, ctxVal.formId, groupId)
    );
    ctx.update((ctx) => {
        return {
            ...ctx,
            groups: ctx.groups.filter((e) => e.id !== groupId),
        };
    });
    await saveGroupsWithPositions(ctx, orgId);
}

export async function syncGroupConfig(
    ctx: Writable<ResponsesContext>,
    orgId: string,
    groupId: string
) {
    const ctxVal = get(ctx);
    const group = ctxVal.groups.find((e) => e.id === groupId);
    if (!group) return;

    await APIs.questionGroups().then((a) =>
        a.questionGroupsSet(orgId, ctxVal.formId, groupId, group)
    );
}

export function qgsIsJump(
    t: APIQuestionGroupStepStrategy
): t is APIQuestionGroupStepStrategyOneOf {
    return !(typeof t === "string");
}

export function getGroupTitle(
    oqpp: boolean,
    formCtx: ResponsesContext,
    group: APIQuestionGroup
) {
    if (oqpp) {
        const q = formCtx.questions.find((e) => e.group_id === group.id);
        if (!q) return "Untitled";
        return q.title;
    } else {
        if (!group.title) {
            return `Section ${group.position + 1}`;
        }
        return group.title;
    }
}
