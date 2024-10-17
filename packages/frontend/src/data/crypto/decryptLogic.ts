import type { APISubmission } from "@paltiverse/palform-typescript-openapi";
import type {
    DecryptedSubmission,
    DecryptedSubmissionBase,
    DecryptedSubmissionSuccess,
} from "./results";
import type {
    decrypt_decode_submission_js,
    KeyResolver,
} from "@paltiverse/palform-crypto";
import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";

export async function decryptAllSubmissionsInternal(
    encryptedSubmissions: APISubmission[],
    keyPEMs: string[],
    successHook: (submission: DecryptedSubmissionSuccess) => Promise<void>,
    allHook: () => void,

    wasm: {
        decrypt_decode_submission_js: typeof decrypt_decode_submission_js;
        KeyResolver: typeof KeyResolver;
    }
) {
    const resolver = new wasm.KeyResolver(keyPEMs);

    const d: DecryptedSubmission[] = [];
    for (let i = 0; i < encryptedSubmissions.length; i++) {
        const sub = encryptedSubmissions[i];
        const base: DecryptedSubmissionBase = {
            id: sub.id,
            createdAt: sub.created_at,
            forToken: sub.for_token ?? null,
        };

        try {
            const result = wasm.decrypt_decode_submission_js(
                sub.data,
                resolver
            ) as InProgressSubmission;
            const submission: DecryptedSubmissionSuccess = {
                ...base,
                questions: result.questions,
                groups: result.groups_completed,
            };
            d.push(submission);
            await successHook(submission);
        } catch (e) {
            d.push({ ...base, error: e as string });
        }

        allHook();
    }
    return d;
}
