import type { InProgressSubmissionRecord } from "../pouch";
import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";
import { APIs } from "../common";
import { createMessage, type Key, readKey } from "openpgp";
import {
    filterKeysByFingerprint,
    getFingerprintsFromURL,
} from "./keyIntegrity";

async function getFormKeys(
    orgId: string,
    formId: string,
    fillAccessToken: string,
    requireFingerprints: boolean
) {
    const resp = await APIs.fill(fillAccessToken).forms.formsKeys(
        orgId,
        formId
    );
    if (resp.data.length === 0) {
        throw new Error("No keys found for organisation.");
    }

    const parsedKeys = await Promise.all(
        resp.data.map((e) => readKey({ armoredKey: e }))
    );

    if (requireFingerprints) {
        const fingerprints = getFingerprintsFromURL(new URL(location.href));
        const filtered = filterKeysByFingerprint(parsedKeys, fingerprints);

        if (filtered.length === 0) {
            throw new Error(
                "Key integrity check failed: no allowed keys were found. Please contact the form owner."
            );
        }

        return filtered;
    }

    return parsedKeys;
}

async function encryptAnything(data: Uint8Array, keys: Key[]) {
    const message = await createMessage({ binary: data });
    const encrypted = await message.encrypt(keys);

    return encrypted;
}

export async function encryptSubmissionAsset(
    file: File,
    orgId: string,
    formId: string,
    fillAccessToken: string,
    requireFingerprints: boolean
) {
    const formKeys = await getFormKeys(
        orgId,
        formId,
        fillAccessToken,
        requireFingerprints
    );

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
    requireFingerprints: boolean,
    captchaValue?: string
) {
    const formKeys = await getFormKeys(
        orgId,
        formId,
        fillAccessToken,
        requireFingerprints
    );

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
