import {
    type APIQuestion,
    type APIQuestionConfiguration,
    type APIQuestionGroup,
    type APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    type ConfigAddress,
    type ConfigChoice,
    type ConfigChoiceMatrix,
    type ConfigDateTime,
    type ConfigFileUpload,
    type ConfigHidden,
    type ConfigInfo,
    type ConfigPhoneNumber,
    type ConfigScale,
    type ConfigSignature,
    type ConfigText,
} from "@palform/palform-typescript-openapi";
import { readable, type Writable, derived, get } from "svelte/store";
import { getContext, setContext } from "svelte";
import { default_question_for_type_js } from "@palform/palform-client-common";
import { APIs } from "../common";
import { randomIdForResource } from "../util/tsid";
import type { FormAdminContext } from "./formAdmin";
import { moveArrayItem, type ArrayMoveDirection } from "../util/arraySwap";

export interface FormEditorContext {
    loading: boolean;
    dirty: boolean;
    questions: Record<string, APIQuestion[]>;
    groups: APIQuestionGroup[];
    currentlyEditing: string | undefined;
}
export function setFormEditorContext(ctx: Writable<FormEditorContext>) {
    setContext("forms-editor", ctx);
}
export function getFormEditorCtx() {
    return getContext<Writable<FormEditorContext>>("forms-editor");
}

export async function syncDirtyForm(
    editorCtx: Writable<FormEditorContext>,
    adminCtx: Writable<FormAdminContext>,
    orgId: string
) {
    const val = get(editorCtx);
    if (!val.dirty) return;

    const flattenedQuestions: APIQuestion[] = [];
    for (const group of val.groups) {
        const questions = val.questions[group.id] ?? [];
        flattenedQuestions.push(...questions);
    }

    await APIs.questions().then((a) =>
        a.questionsSave(orgId, get(adminCtx).formId, {
            questions: flattenedQuestions,
            groups: val.groups,
        })
    );
    editorCtx.update((e) => ({ ...e, dirty: false }));
    adminCtx.update((e) => ({
        ...e,
        questions: flattenedQuestions,
        groups: val.groups,
    }));
}

export function initialiseEditorContext(
    ctx: Writable<FormEditorContext>,
    adminCtx: FormAdminContext
) {
    const groupedQuestions: Record<string, APIQuestion[]> = {};
    for (const question of adminCtx.questions) {
        if (!(question.group_id in groupedQuestions)) {
            groupedQuestions[question.group_id] = [];
        }

        groupedQuestions[question.group_id].push(question);
    }
    ctx.update((ctx) => {
        return {
            ...ctx,
            questions: groupedQuestions,
            groups: adminCtx.groups,
            dirty: false,
        };
    });
}

export function getEditorQuestion(questionId: string) {
    return derived(
        [readable(questionId), getFormEditorCtx()],
        ([questionId, ctx]) => {
            return Object.values(ctx.questions)
                .flat()
                .find((e) => e.id === questionId);
        }
    );
}
export function getEditorQuestionGroup(groupId: string) {
    return derived(
        [readable(groupId), getFormEditorCtx()],
        ([groupId, ctx]) => {
            return ctx.groups.find((e) => e.id === groupId);
        }
    );
}
export function getEditorQuestionsInGroup(groupId: string) {
    return derived(
        [readable(groupId), getFormEditorCtx()],
        ([groupId, ctx]) => {
            return ctx.questions[groupId] ?? [];
        }
    );
}

export function insertQuestion(
    ctx: Writable<FormEditorContext>,
    questionType: string,
    position: number,
    groupId: string
) {
    const question: APIQuestion = {
        id: randomIdForResource("question"),
        title: "Untitled",
        description: null,
        internal_name: null,
        required: false,
        configuration: default_question_for_type_js(questionType),
        group_id: groupId,
    };
    ctx.update((ctx) => {
        if (!(groupId in ctx.questions)) {
            ctx.questions[groupId] = [];
        }

        ctx.questions[groupId].splice(position, 0, question);
        ctx.dirty = true;
        return ctx;
    });

    return question.id;
}

export function insertQuestionGroup(
    ctx: Writable<FormEditorContext>,
    beforeIndex: number,
    title: string | null,
    description: string | null
) {
    const obj: APIQuestionGroup = {
        title,
        description,
        id: randomIdForResource("question_group"),
        step_strategy: "NextPosition",
    };
    ctx.update((ctx) => {
        ctx.groups.splice(beforeIndex, 0, {
            ...obj,
        });
        ctx.questions[obj.id] = [];
        ctx.dirty = true;
        return ctx;
    });

    return obj.id;
}

export function updateQuestion(
    ctx: Writable<FormEditorContext>,
    question: APIQuestion
) {
    ctx.update((ctx) => {
        const i = ctx.questions[question.group_id].findIndex(
            (e) => e.id === question.id
        );
        ctx.questions[question.group_id][i] = question;
        ctx.dirty = true;
        return ctx;
    });
}

export function updateQuestionGroup(
    ctx: Writable<FormEditorContext>,
    group: APIQuestionGroup
) {
    ctx.update((ctx) => {
        const i = ctx.groups.findIndex((e) => e.id === group.id);
        if (i === -1) return ctx;
        ctx.groups[i] = group;
        ctx.dirty = true;
        return ctx;
    });
}

export function moveQuestionGroup(
    ctx: Writable<FormEditorContext>,
    group: APIQuestionGroup,
    direction: ArrayMoveDirection
) {
    ctx.update((ctx) => {
        moveArrayItem(ctx.groups, group, direction);
        ctx.dirty = true;
        return ctx;
    });
}

export function moveQuestion(
    ctx: Writable<FormEditorContext>,
    question: APIQuestion,
    direction: ArrayMoveDirection
) {
    const groupId = question.group_id;
    ctx.update((ctx) => {
        moveArrayItem(ctx.questions[groupId], question, direction);
        ctx.dirty = true;
        return ctx;
    });
}

export function deleteQuestion(
    ctx: Writable<FormEditorContext>,
    groupId: string,
    questionId: string
) {
    ctx.update((ctx) => {
        ctx.questions[groupId] = ctx.questions[groupId].filter(
            (e) => e.id !== questionId
        );

        ctx.dirty = true;
        return ctx;
    });
}

export function deleteGroup(ctx: Writable<FormEditorContext>, groupId: string) {
    ctx.update((ctx) => {
        delete ctx.questions[groupId];
        return {
            ...ctx,
            dirty: true,
            groups: ctx.groups.filter((e) => e.id !== groupId),
        };
    });
}

export type StrategyMatcherEventProps = {
    onsave: (
        matcher: APIQuestionGroupStepStrategyJumpCaseConditionMatcher
    ) => void;
};

function extractQuestionTypeLabel(config: APIQuestionConfiguration) {
    return Object.keys(config)[0];
}

export function humanQuestionTypeLabel(config: APIQuestionConfiguration) {
    if (qIsInfo(config)) {
        return "Info";
    } else if (qIsText(config)) {
        return "Text";
    } else if (qIsChoice(config)) {
        return "Choice";
    } else if (qIsScale(config)) {
        return "Scale";
    } else if (qIsAddress(config)) {
        return "Address";
    } else if (qIsPhoneNumber(config)) {
        return "Phone number";
    } else if (qIsFileUpload(config)) {
        return "File upload";
    } else if (qIsSignature(config)) {
        return "Signature";
    } else if (qIsChoiceMatrix(config)) {
        return "Choice matrix";
    } else if (qIsDateTime(config)) {
        return "Date/time";
    } else if (qIsHidden(config)) {
        return "Hidden";
    }
}

function qIs<T extends APIQuestionConfiguration>(
    label: string
): (config?: APIQuestionConfiguration) => config is T {
    return (config?: APIQuestionConfiguration): config is T =>
        config !== undefined && extractQuestionTypeLabel(config) === label;
}
export const qIsInfo = qIs<ConfigInfo>("info");
export const qIsText = qIs<ConfigText>("text");
export const qIsChoice = qIs<ConfigChoice>("choice");
export const qIsScale = qIs<ConfigScale>("scale");
export const qIsAddress = qIs<ConfigAddress>("address");
export const qIsPhoneNumber = qIs<ConfigPhoneNumber>("phone_number");
export const qIsFileUpload = qIs<ConfigFileUpload>("file_upload");
export const qIsSignature = qIs<ConfigSignature>("signature");
export const qIsChoiceMatrix = qIs<ConfigChoiceMatrix>("choice_matrix");
export const qIsDateTime = qIs<ConfigDateTime>("date_time");
export const qIsHidden = qIs<ConfigHidden>("hidden");
export const qIsMeta = (config?: APIQuestionConfiguration) => qIsInfo(config);
