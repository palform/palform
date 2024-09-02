import type { InProgressSubmission } from "@paltiverse/palform-client-js-extra-types/InProgressSubmission";
import PouchDB from "pouchdb-browser";
import PouchDBFind from "pouchdb-find";
import type { DecryptedSubmissionSuccess } from "./crypto/results";

export interface AuthTokenRecord {
    _id: string;
    tokenSecret: string;
    created: number;
    expires: number;
}

PouchDB.plugin(PouchDBFind);
export const authDb = new PouchDB<AuthTokenRecord>("palform_auth");
await authDb.createIndex({
    index: {
        fields: ["expires"],
    },
});

export type InProgressSubmissionRecord = Omit<
    InProgressSubmission,
    "form_id"
> & {
    _id: string;
    _rev?: string;
};
export const formFillDb = new PouchDB<InProgressSubmissionRecord>(
    "palform_form_fill",
    { auto_compaction: true }
);

export interface CryptoKeyRecord {
    _id: string;
    _rev?: string;
    privateKey: string;
    serverId: string;
    orgId: string;
    userId: string;
}
export const privateKeyDb = new PouchDB<CryptoKeyRecord>("palform_private_key");
privateKeyDb.createIndex({
    index: {
        fields: ["serverId"],
    },
});

export interface DecryptedSubmissionCacheRecord {
    _id: string;
    _rev?: string;
    orgId: string;
    formId: string;
    submission: DecryptedSubmissionSuccess;
    created: number;
}
export const decryptedSubmissionCacheDb =
    new PouchDB<DecryptedSubmissionCacheRecord>(
        "palform_decrypted_submission_cache",
        { auto_compaction: true }
    );
decryptedSubmissionCacheDb.createIndex({
    index: {
        fields: ["formId"],
    },
});
decryptedSubmissionCacheDb.createIndex({
    index: {
        fields: ["created"],
    },
});

export interface OrgVisitRecord {
    _id: string;
    _rev?: string;
    time: number;
}
export const orgVisitDb = new PouchDB<OrgVisitRecord>("palform_org_visit", {
    auto_compaction: true,
});
