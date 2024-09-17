import type { APISubmission } from "@paltiverse/palform-typescript-openapi";
import type {
    DecryptedSubmission,
    DecryptedSubmissionBase,
    DecryptedSubmissionSuccess,
} from "./results";
import * as Comlink from "comlink";
import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";

async function decryptAllSubmissions(
    encryptedSubmissions: APISubmission[],
    keyPEMs: string[],
    statusUpdate: () => void,
    cacheSubmission: (submission: DecryptedSubmissionSuccess) => Promise<void>
) {
    const { decrypt_decode_submission_js, KeyResolver } = await import(
        "@paltiverse/palform-crypto"
    );

    const resolver = new KeyResolver(keyPEMs);

    const d: DecryptedSubmission[] = [];
    for (let i = 0; i < encryptedSubmissions.length; i++) {
        const sub = encryptedSubmissions[i];
        const base: DecryptedSubmissionBase = {
            id: sub.id,
            createdAt: sub.created_at,
            forToken: sub.for_token ?? null,
        };

        try {
            const result = decrypt_decode_submission_js(
                sub.data,
                resolver
            ) as InProgressSubmission;
            const submission: DecryptedSubmissionSuccess = {
                ...base,
                questions: result.questions,
                groups: result.groups_completed,
            };
            d.push(submission);
            await cacheSubmission(submission);
        } catch (e) {
            d.push({ ...base, error: (e as Error).message });
        }

        statusUpdate();
    }
    return d;
}

Comlink.expose(decryptAllSubmissions);
export type DecryptAllSubmissionsFunction = typeof decryptAllSubmissions;
