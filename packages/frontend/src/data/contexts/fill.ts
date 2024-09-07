import { derived, get, readable, writable } from "svelte/store";
import { formFillDb, type InProgressSubmissionRecord } from "../pouch";
import { APIs } from "../common";
import {
    type ValidationError,
    api_question_default_submission,
    next_question_group_step_js,
    question_submission_data_to_string_js,
    question_submission_is_empty_js,
    validate_questions_js,
} from "@paltiverse/palform-client-common";
import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
import { Mutex } from "async-mutex";
import type { APIFormWithQuestions } from "@paltiverse/palform-typescript-openapi";
import { qIsInfo } from "../contexts/questionsEditing";

const formFillSaveMutex = new Mutex();
export interface FormFillContext {
    submission: InProgressSubmissionRecord;
    form: APIFormWithQuestions;
    organisationId: string;
    currentGroupId: string;
    fillAccessToken: string;
}
export const formFillStore = writable<FormFillContext | undefined>(undefined);
export function ctxGetCurrentGroup() {
    return derived([formFillStore], ([formFillStore]) => {
        return formFillStore?.form.g.find(
            (e) => e.id === formFillStore.currentGroupId
        );
    });
}
export function ctxGetCurrentGroupQuestions() {
    return derived([formFillStore], ([formFillStore]) => {
        return (
            formFillStore?.form.q.filter(
                (e) => e.group_id === formFillStore.currentGroupId
            ) ?? []
        );
    });
}
export function ctxGetNextStep() {
    return derived([formFillStore], ([formFillStore]) => {
        if (!formFillStore) return;
        const next_group = next_question_group_step_js(
            formFillStore.currentGroupId,
            formFillStore.form.g,
            formFillStore.submission.questions
        );
        return next_group;
    });
}

export const questionValidationStore = writable<ValidationError[]>([]);

export interface FillSendState {
    loading: boolean;
    error: string | undefined;
    done: boolean;
}
export const fillSendStore = writable<FillSendState | undefined>(undefined);

export async function loadFormFillFromShortLink(
    subdomain: string,
    shortLink: string
) {
    const resp = await APIs.formsNoAuth.formsExchangeShortLink(
        subdomain,
        shortLink
    );
    return loadFormFill(
        resp.data.org_id,
        resp.data.form_id,
        resp.data.fill_token_id
    );
}

export async function loadFormFill(
    orgId: string,
    formId: string,
    fillAccessToken: string
) {
    const resp = await APIs.fill(fillAccessToken).forms.formsView(
        orgId,
        formId
    );

    const release = await formFillSaveMutex.acquire();
    let formFill: InProgressSubmissionRecord;
    try {
        formFill = await formFillDb.get(formId);
    } catch (e) {
        formFill = {
            _id: formId,
            groups_completed: [],
            questions: [],
        };
    }

    for (const question of resp.data.q) {
        if (qIsInfo(question.configuration)) continue;

        if (
            // if we don't already have an entry for this question
            !formFill.questions.some((e) => e.question_id === question.id)
        ) {
            formFill.questions.push({
                question_id: question.id,
                data: api_question_default_submission(question),
            });
        }
    }

    if (resp.data.g.length === 0 || resp.data.q.length === 0) {
        release();
        throw new Error("This form is empty");
    }

    formFill.groups_completed = [];

    const putResp = await formFillDb.put(formFill);
    formFillStore.set({
        submission: {
            ...formFill,
            _rev: putResp.rev,
        },
        form: resp.data,
        organisationId: orgId,
        currentGroupId: resp.data.g[0].id,
        fillAccessToken,
    });
    release();
}

export async function saveFormFill() {
    const fill = get(formFillStore);
    if (!fill) return;

    const release = await formFillSaveMutex.acquire();
    const resp = await formFillDb.put(fill.submission);
    formFillStore.update((fill) => {
        if (!fill) return fill;
        fill.submission._rev = resp.rev;
        return fill;
    });
    release();
}

export const selectQuestion = (questionId: string) =>
    derived([readable(questionId), formFillStore], ([$id, $currentFill]) => {
        return $currentFill?.submission.questions.find(
            (e) => e.question_id === $id
        )?.data;
    });

export function setQuestionValue(
    questionId: string,
    value: QuestionSubmissionData
) {
    formFillStore.update((fill) => {
        if (fill === undefined) throw new Error();
        const question = fill.submission.questions.find(
            (e) => e.question_id === questionId
        );
        if (!question) throw new Error();
        question.data = value;
        return fill;
    });
}

export async function deleteFormFill() {
    const currentFill = get(formFillStore);
    if (!currentFill || !currentFill.submission._rev) return;

    const release = await formFillSaveMutex.acquire();
    await formFillDb.remove({
        _id: currentFill.submission._id,
        _rev: currentFill.submission._rev,
    });
    release();
}

export function validateQuestions() {
    const currentFill = get(formFillStore);
    if (!currentFill) return;

    const currentGroupQuestions = get(ctxGetCurrentGroupQuestions());

    const errors = validate_questions_js(
        currentGroupQuestions,
        currentFill.submission.questions
    );
    questionValidationStore.set(errors);
    return errors.length === 0;
}

export function selectQuestionValidationErrors(questionId: string) {
    return derived(
        [readable(questionId), questionValidationStore],
        ([_, validations]) =>
            validations.find((e) => e.question_id === questionId)
    );
}

export function templateFillQuestionText(text: string) {
    const currentFill = get(formFillStore);
    if (!currentFill) return text;

    const matches = [...text.matchAll(/{{\w+}}/g)];
    let newText = `${text}`;

    for (const match of matches) {
        const withoutStart = match[0].substring(2);
        const identifier = withoutStart.substring(0, withoutStart.length - 2);

        const matchingQuestion = currentFill.form.q.find(
            (e) => e.internal_name === identifier
        );
        if (!matchingQuestion) continue;

        let replacedValue = "";

        const matchingSubmission = currentFill.submission.questions.find(
            (e) => e.question_id === matchingQuestion.id
        );
        if (matchingSubmission) {
            replacedValue = question_submission_data_to_string_js(
                matchingSubmission.data
            );
        }

        newText = [
            newText.substring(0, match.index),
            replacedValue,
            newText.substring(match.index + match[0].length),
        ].join("");
    }

    return newText;
}

export function sGetText(s: QuestionSubmissionData) {
    if ("Text" in s) {
        return s.Text;
    }
    throw new Error();
}
export function sGetChoice(s: QuestionSubmissionData) {
    if ("Choice" in s) {
        return s.Choice;
    }
    throw new Error();
}
export function sGetScale(s: QuestionSubmissionData) {
    if ("Scale" in s) {
        return s.Scale;
    }
    throw new Error();
}
export function sGetAddress(s: QuestionSubmissionData) {
    if ("Address" in s) {
        return s.Address;
    }
    throw new Error();
}
export function sGetPhoneNumber(s: QuestionSubmissionData) {
    if ("PhoneNumber" in s) {
        return s.PhoneNumber;
    }
    throw new Error();
}
export function sGetFileUpload(s: QuestionSubmissionData) {
    if ("FileUpload" in s) {
        return s.FileUpload;
    }
    throw new Error();
}
export function sGetSignature(s: QuestionSubmissionData) {
    if ("Signature" in s) {
        return s.Signature;
    }
    throw new Error();
}
export function sGetChoiceMatrix(s: QuestionSubmissionData) {
    if ("ChoiceMatrix" in s) {
        return s.ChoiceMatrix;
    }
    throw new Error();
}
export function sGetDateTime(s: QuestionSubmissionData) {
    if ("DateTime" in s) {
        return s.DateTime;
    }
    throw new Error();
}
export function sGetHidden(s: QuestionSubmissionData) {
    if ("Hidden" in s) {
        return s.Hidden;
    }
    throw new Error();
}
export function sIsNonEmpty(s: QuestionSubmissionData) {
    return !question_submission_is_empty_js(s);
}
