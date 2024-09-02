import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";
import type { APIQuestion } from "@paltiverse/palform-typescript-openapi";
import {
    submissionIsSuccess,
    type DecryptedSubmission,
} from "../../crypto/results";
import { FormAnalysisManager } from "@paltiverse/palform-analysis";
import { getContext, setContext } from "svelte";
import { derived, readable, type Writable } from "svelte/store";

const CORRELATION_THRESHOLD = 0.75;

export interface AnalysisCorrelationContext {
    manager: FormAnalysisManager | null;
    correlations: Map<string, Map<string, Map<string, Map<string, number>>>>;
}

export function initCorrelationContext(
    formId: string,
    questions: APIQuestion[],
    submissions: DecryptedSubmission[],
    ctx: Writable<AnalysisCorrelationContext>
) {
    const ips = submissions
        .filter((e) => submissionIsSuccess(e))
        .map((e) => {
            return {
                form_id: formId,
                groups_completed: e.groups,
                questions: e.questions,
            } as InProgressSubmission;
        });

    if (ips.length === 0 || questions.length === 0) {
        ctx.set({
            manager: null,
            correlations: new Map(),
        });
        return;
    }

    const manager = new FormAnalysisManager(questions, ips);
    const correlations = manager.question_influencers();

    ctx.set({
        manager,
        correlations,
    });
}
export function setCorrelationContext(
    ctx: Writable<AnalysisCorrelationContext>
) {
    setContext("analysis_correlation", ctx);
}
export function getCorrelationContext() {
    return getContext<Writable<AnalysisCorrelationContext>>(
        "analysis_correlation"
    );
}
export function getCorrelationsForQuestion(questionId: string) {
    const ctx = getContext<Writable<AnalysisCorrelationContext>>(
        "analysis_correlation"
    );

    return derived([readable(questionId), ctx], ([questionId, ctx]) => {
        const vals = ctx.correlations.get(questionId);
        if (!vals) return undefined;
        return Array.from(vals.entries()).map(([fromFeatureLabel, e]) => {
            const a = Array.from(e.entries());
            return [
                fromFeatureLabel,
                a.map(([toQuestionId, e]) => {
                    const b = Array.from(e.entries());
                    return [
                        toQuestionId,
                        b.filter(
                            ([_, v]) =>
                                !Number.isNaN(v) &&
                                Math.abs(v) >= CORRELATION_THRESHOLD
                        ),
                    ] as [string, [string, number][]];
                }),
            ] as [string, [string, [string, number][]][]];
        });
    });
}

export function getFeatureGraphData(
    ctx: AnalysisCorrelationContext,
    fromQuestionId: string,
    fromFeatureLabel: string,
    toQuestionId: string,
    toFeatureLabel: string
) {
    if (!ctx.manager) return;
    const resp = ctx.manager.regression_for_question_pair(
        fromQuestionId,
        fromFeatureLabel,
        toQuestionId,
        toFeatureLabel
    );
    return {
        points: resp.points as number[][],
        gradient: resp.gradient,
        intercept: resp.intercept,
    };
}
