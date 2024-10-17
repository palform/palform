import type { APISubmission } from "@paltiverse/palform-typescript-openapi";
import type { DecryptedSubmissionSuccess } from "./results";
import * as Comlink from "comlink";
import { decryptAllSubmissionsInternal } from "./decryptLogic";

async function decryptAllSubmissions(
    encryptedSubmissions: APISubmission[],
    keyPEMs: string[],
    statusUpdate: () => void,
    cacheSubmission: (submission: DecryptedSubmissionSuccess) => Promise<void>
) {
    const { decrypt_decode_submission_js, KeyResolver } = await import(
        "@paltiverse/palform-crypto"
    );

    return await decryptAllSubmissionsInternal(
        encryptedSubmissions,
        keyPEMs,
        cacheSubmission,
        statusUpdate,
        { decrypt_decode_submission_js, KeyResolver }
    );
}

Comlink.expose(decryptAllSubmissions);
export type DecryptAllSubmissionsFunction = typeof decryptAllSubmissions;
