import {
    type APIQuestionConfigurationOneOf3,
    type APIQuestion,
    type APIQuestionConfiguration,
    type APIQuestionConfigurationOneOf,
    type APIQuestionConfigurationOneOf1,
    type APIQuestionConfigurationOneOf2,
    type APIQuestionConfigurationOneOf4,
    type APIQuestionConfigurationOneOf5,
    type APIQuestionConfigurationOneOf6,
    type APIQuestionConfigurationOneOf7,
    type APIQuestionConfigurationOneOf8,
    type APIQuestionConfigurationOneOf9,
    type APIQuestionConfigurationOneOf10,
} from "@paltiverse/palform-typescript-openapi";
import { readable, type Writable, derived, get } from "svelte/store";
import { getContext, setContext } from "svelte";
import { APIs } from "../common";

export interface EditorContext {
    loading: boolean;
    questions: Record<string, APIQuestion[]>;
    currentlyEditing: string | undefined;
}
export function setEditorContext(ctx: Writable<EditorContext>) {
    setContext("forms-editor", ctx);
}
export function getEditorCtx() {
    return getContext<Writable<EditorContext>>("forms-editor");
}
export function setQuestionsInEditorContext(
    ctx: Writable<EditorContext>,
    questions: APIQuestion[]
) {
    const groupedQuestions: Record<string, APIQuestion[]> = {};
    for (const question of questions) {
        if (!(question.group_id in groupedQuestions)) {
            groupedQuestions[question.group_id] = [];
        }

        groupedQuestions[question.group_id].push(question);
    }
    ctx.update((ctx) => {
        return {
            ...ctx,
            questions: groupedQuestions,
        };
    });
}

export function getEditorQuestion(questionId: string) {
    return derived(
        [readable(questionId), getEditorCtx()],
        ([questionId, ctx]) => {
            return Object.values(ctx.questions)
                .flat()
                .find((e) => e.id === questionId);
        }
    );
}
export function getEditorQuestionsInGroup(groupId: string) {
    return derived([readable(groupId), getEditorCtx()], ([groupId, ctx]) => {
        return ctx.questions[groupId] ?? [];
    });
}

export async function saveQuestionsWithPositions(
    ctx: Writable<EditorContext>,
    orgId: string,
    formId: string,
    onlyGroupId?: string,
    ignoreQuestions: string[] = []
) {
    const api = await APIs.questions();
    const currentCtx = get(ctx);
    for (const [groupId, groupQuestions] of Object.entries(
        currentCtx.questions
    )) {
        if (onlyGroupId && onlyGroupId !== groupId) continue;
        for (let i = 0; i < groupQuestions.length; i++) {
            const q = groupQuestions[i];
            if (ignoreQuestions.includes(q.id)) continue;
            await api.questionsSet(orgId, formId, q.group_id, q.id, {
                ...q,
                position: i,
            });
            ctx.update((c) => {
                const matchingQuestion = c.questions[groupId].findIndex(
                    (e) => e.id === q.id
                );
                if (matchingQuestion === -1) return c;
                c.questions[groupId][matchingQuestion].position = i;
                return c;
            });
        }
    }
}
export async function insertQuestion(
    ctx: Writable<EditorContext>,
    questionType: string,
    position: number,
    orgId: string,
    formId: string,
    groupId: string
) {
    const api = await APIs.questions();
    const newQuestionResp = await api.questionsCreate(orgId, formId, groupId, {
        question_type: questionType,
        position,
    });

    const question = newQuestionResp.data;
    ctx.update((ctx) => {
        if (!(groupId in ctx.questions)) {
            ctx.questions[groupId] = [];
        }

        ctx.questions[groupId].splice(position, 0, question);
        return ctx;
    });
    try {
        await saveQuestionsWithPositions(ctx, orgId, formId, groupId, [
            question.id,
        ]);
    } catch (e) {
        deleteLocalQuestion(ctx, question.id);
        throw e;
    }

    return question.id;
}
export function deleteLocalQuestion(
    ctx: Writable<EditorContext>,
    questionId: string
) {
    ctx.update((ctx) => {
        for (const groupId of Object.keys(ctx.questions)) {
            ctx.questions[groupId] = ctx.questions[groupId].filter(
                (e) => e.id !== questionId
            );
        }
        return ctx;
    });
}
export async function deleteQuestion(
    ctx: Writable<EditorContext>,
    orgId: string,
    formId: string,
    groupId: string,
    questionId: string
) {
    await APIs.questions().then((a) =>
        a.questionsDelete(orgId, formId, groupId, questionId)
    );
    deleteLocalQuestion(ctx, questionId);
    await saveQuestionsWithPositions(ctx, orgId, formId);
}

export type QuestionEditEvents = { update: APIQuestionConfiguration };

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
): (config: APIQuestionConfiguration) => config is T {
    return (config: APIQuestionConfiguration): config is T =>
        extractQuestionTypeLabel(config) === label;
}
export const qIsInfo = qIs<APIQuestionConfigurationOneOf>("info");
export const qIsText = qIs<APIQuestionConfigurationOneOf1>("text");
export const qIsChoice = qIs<APIQuestionConfigurationOneOf2>("choice");
export const qIsScale = qIs<APIQuestionConfigurationOneOf3>("scale");
export const qIsAddress = qIs<APIQuestionConfigurationOneOf4>("address");
export const qIsPhoneNumber =
    qIs<APIQuestionConfigurationOneOf5>("phone_number");
export const qIsFileUpload = qIs<APIQuestionConfigurationOneOf6>("file_upload");
export const qIsSignature = qIs<APIQuestionConfigurationOneOf7>("signature");
export const qIsChoiceMatrix =
    qIs<APIQuestionConfigurationOneOf8>("choice_matrix");
export const qIsDateTime = qIs<APIQuestionConfigurationOneOf9>("date_time");
export const qIsHidden = qIs<APIQuestionConfigurationOneOf10>("hidden");
export const qIsMeta = (config: APIQuestionConfiguration) => qIsInfo(config);
