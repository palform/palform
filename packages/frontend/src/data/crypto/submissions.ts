import {
    decrypt_blob_js,
    encrypt_blob_js,
    encrypt_submission_js,
    KeyResolver,
} from "@paltiverse/palform-client-common";
import type { InProgressSubmissionRecord } from "../pouch";
import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";
import { APIs } from "../common";
import { getPrivateKeys } from "../contexts/keys";

async function getFormKeys(
    orgId: string,
    formId: string,
    fillAccessToken: string
) {
    const resp = await APIs.fill(fillAccessToken).forms.formsKeys(
        orgId,
        formId
    );
    if (resp.data.length === 0) {
        throw new Error("No keys found for organisation.");
    }
    return resp.data;
}

export async function encryptSubmissionAsset(
    file: File,
    orgId: string,
    formId: string,
    fillAccessToken: string
) {
    const formKeys = await getFormKeys(orgId, formId, fillAccessToken);
    if (formKeys.length === 0) {
        throw new Error(
            "No public keys returned from server; cannot encrypt submission"
        );
    }

    const fileData = new Uint8Array(await file.arrayBuffer());
    return encrypt_blob_js(fileData, formKeys);
}

export async function decryptSubmissionAsset(data: string) {
    const keyPEMs = await getPrivateKeys();
    const resolver = new KeyResolver(keyPEMs);
    return decrypt_blob_js(data, resolver);
}

export async function sendSubmission(
    submission: InProgressSubmissionRecord,
    orgId: string,
    formId: string,
    fillAccessToken: string,
    lastGroupId: string,
    captchaValue?: string
) {
    const formKeys = await getFormKeys(orgId, formId, fillAccessToken);
    if (formKeys.length === 0) {
        throw new Error(
            "No public keys returned from server; cannot encrypt submission"
        );
    }

    const submissionToEncrypt: InProgressSubmission = {
        form_id: submission._id,
        questions: submission.questions,
        groups_completed: [...submission.groups_completed, lastGroupId],
    };
    const encryptedSubmission = encrypt_submission_js(
        submissionToEncrypt,
        formKeys
    );
    await APIs.fill(fillAccessToken).forms.formsFill(
        orgId,
        formId,
        encryptedSubmission,
        captchaValue,
    );
}
