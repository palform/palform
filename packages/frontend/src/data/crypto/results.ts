import type { QuestionSubmission } from "@paltiverse/palform-client-js-extra-types/QuestionSubmission";
import type { APISubmission } from "@paltiverse/palform-typescript-openapi";
import DecryptWorker from "./decryptWorker?worker";
import type { Readable, Writable } from "svelte/store";
import type { ResponsesContext } from "../contexts/results";
import * as Comlink from "comlink";
import type { DecryptAllSubmissionsFunction } from "./decryptWorker";
import { getPrivateKeys } from "../contexts/keys";
import { decryptedSubmissionCacheDb, pouchInfiniteLimit } from "../pouch";
import { DateTime } from "luxon";
import { APIs } from "../common";
import { parseServerTime } from "../util/time";

export interface DecryptedSubmissionBase {
    id: string;
    createdAt: string;
    forToken: string | null;
}
export interface DecryptedSubmissionSuccess extends DecryptedSubmissionBase {
    questions: QuestionSubmission[];
    groups: string[];
}
export interface DecryptedSubmissionError extends DecryptedSubmissionBase {
    error: string;
}
export type DecryptedSubmission =
    | DecryptedSubmissionSuccess
    | DecryptedSubmissionError;

function splitSubmissions(submissions: APISubmission[]) {
    if (submissions.length < 100) return [submissions];
    const groupCount = navigator.hardwareConcurrency ?? 2;
    const groups: APISubmission[][] = [];
    for (let i = 0; i < groupCount; i++) {
        groups.push([]);
    }

    for (let i = 0; i < submissions.length; i++) {
        const groupIndex = i % groupCount;
        groups[groupIndex].push(submissions[i]);
    }

    return groups;
}

export async function downloadSubmissionsForForm(
    orgId: string,
    formId: string,
    statusUpdate: Writable<{ total: number; done: number } | undefined>,
    responsesCtx: Writable<ResponsesContext>,
    terminateHandle: Readable<boolean>
) {
    const allCachedSubmissions = (
        await decryptedSubmissionCacheDb.find({
            selector: {
                formId,
                created: { $gt: null },
            },
            sort: [{ created: "asc" }],
            limit: pouchInfiniteLimit,
        })
    ).docs;
    const latestCachedSubmission =
        allCachedSubmissions.length > 0
            ? allCachedSubmissions[allCachedSubmissions.length - 1]
            : undefined;

    const submissionsResp = await APIs.submissions().then((a) =>
        a.submissionsList(orgId, formId, latestCachedSubmission?._id)
    );
    const sStream = submissionsResp.data;

    for (const deleted of sStream.deleted) {
        const thisSubmission = allCachedSubmissions.find(
            (e) => e._id === deleted
        );
        if (!thisSubmission) continue;
        await decryptedSubmissionCacheDb.remove(thisSubmission);
        allCachedSubmissions.splice(
            allCachedSubmissions.indexOf(thisSubmission),
            1
        );
    }

    statusUpdate.set({
        total: submissionsResp.data.total,
        done: allCachedSubmissions.length,
    });

    const worker = new DecryptWorker();
    const workerWrap = Comlink.wrap<DecryptAllSubmissionsFunction>(worker);
    const unsub = terminateHandle.subscribe((terminate) => {
        if (terminate) {
            worker.terminate();
            unsub();
        }
    });

    const submissionGroups = splitSubmissions(sStream.new);
    const keyPEMs = await getPrivateKeys();

    const resp = await Promise.all(
        submissionGroups.map((group) =>
            workerWrap(
                group,
                keyPEMs,
                Comlink.proxy(() => {
                    statusUpdate.update((ctx) => {
                        if (!ctx) return undefined;
                        return {
                            ...ctx,
                            done: ctx.done + 1,
                        };
                    });
                }),
                Comlink.proxy(async (submission) => {
                    await decryptedSubmissionCacheDb.put({
                        _id: submission.id,
                        orgId,
                        formId,
                        submission,
                        created: parseServerTime(
                            submission.createdAt,
                            true
                        ).toMillis(),
                    });
                })
            )
        )
    );

    const flatResp = resp.flat(1);
    flatResp.push(...allCachedSubmissions.map((e) => e.submission));
    flatResp.sort((a, b) => {
        const aDate = DateTime.fromISO(a.createdAt);
        const bDate = DateTime.fromISO(b.createdAt);
        return aDate === bDate ? 0 : aDate < bDate ? -1 : 1;
    });

    responsesCtx.update((ctx) => {
        return { ...ctx, submissions: flatResp };
    });
}

export function submissionIsError(
    s: DecryptedSubmission
): s is DecryptedSubmissionError {
    return "error" in s;
}
export function submissionIsSuccess(
    s: DecryptedSubmission
): s is DecryptedSubmissionSuccess {
    return "questions" in s;
}
