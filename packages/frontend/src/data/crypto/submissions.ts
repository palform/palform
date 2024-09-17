import type { InProgressSubmissionRecord } from "../pouch";
import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";
import { APIs } from "../common";
import { createMessage, readKey } from "openpgp";

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

async function encryptAnything(data: Uint8Array, keys: string[]) {
    const parsedKeys = await Promise.all(
        keys.map((e) => readKey({ armoredKey: e }))
    );

    const message = await createMessage({ binary: data });
    const encrypted = await message.encrypt(parsedKeys);

    return encrypted;
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
    const encryptedData = await encryptAnything(fileData, formKeys);
    return new TextEncoder().encode(encryptedData.armor());
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

    const encodedSubmission = new TextEncoder().encode(
        JSON.stringify(submissionToEncrypt)
    );
    const encryptedSubmission = await encryptAnything(
        encodedSubmission,
        formKeys
    );

    await APIs.fill(fillAccessToken).forms.formsFill(
        orgId,
        formId,
        encryptedSubmission.armor(),
        captchaValue
    );
}
