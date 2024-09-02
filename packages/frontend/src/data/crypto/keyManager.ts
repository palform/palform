import { getUserId } from "../auth";
import { APIs } from "../common";
import { privateKeyDb, type CryptoKeyRecord } from "../pouch";
import {
    decrypt_backed_up_key_js,
    encrypt_key_for_backup_js,
    generate_certificate_js,
    get_key_metadata_js,
    strip_secret_bits_js,
} from "@paltiverse/palform-client-common";
import downloadFile from "../util/downloadFile";

export async function findKey(
    serverId: string,
): Promise<CryptoKeyRecord | null> {
    const resp = await privateKeyDb.find({
        selector: {
            serverId,
        },
    });
    return resp.docs[0] ?? null;
}

export async function registerKey(expiryDays: number, orgId: string) {
    const userId = await getUserId();
    if (!userId) return;

    const expirySeconds = expiryDays * 24 * 60 * 60;
    const newKeypair = generate_certificate_js(orgId, userId, expirySeconds);

    const serverIdResp = await APIs.keys().then((a) =>
        a.keysRegister(orgId, {
            key_data: newKeypair.public,
        }),
    );

    await privateKeyDb.put({
        _id: newKeypair.key_id,
        privateKey: newKeypair.private,
        serverId: serverIdResp.data,
        orgId,
        userId,
    });

    return serverIdResp.data;
}

export async function importKey(privateKeyPEM: string, orgId: string) {
    const userId = await getUserId();
    if (!userId) return;

    const metadata = get_key_metadata_js(privateKeyPEM);
    if (!metadata.has_secret)
        throw new Error("Key does not have secret material");

    const safePublicKey = strip_secret_bits_js(privateKeyPEM);
    const serverIdResp = await APIs.keys().then((a) =>
        a.keysRegister(orgId, {
            key_data: safePublicKey,
        }),
    );

    await privateKeyDb.put({
        _id: metadata.fingerprint,
        privateKey: privateKeyPEM,
        serverId: serverIdResp.data,
        orgId,
        userId,
    });
    return serverIdResp.data;
}

export function downloadPrivateKey(record: CryptoKeyRecord) {
    downloadFile(`${record.serverId}.asc`, record.privateKey);
}

export async function deleteLocalKey(record: CryptoKeyRecord) {
    if (record._rev === undefined) return;
    await privateKeyDb.remove({
        _rev: "",
        ...record,
    });
}

export async function backupKey(
    orgId: string,
    serverId: string,
    passphrase: string,
) {
    const keyRecord = await findKey(serverId);
    if (!keyRecord) throw new Error(`Key with server ID ${serverId} not found`);

    const result = encrypt_key_for_backup_js(keyRecord.privateKey, passphrase);
    await APIs.keys().then((a) =>
        a.keysRegisterBackup(orgId, serverId, {
            key_data: result,
        }),
    );
}

export async function restoreKeyFromBackup(
    orgId: string,
    serverId: string,
    passphrase: string,
) {
    const userId = await getUserId();
    if (!userId) return;

    const encryptedBackupResp = await APIs.keys().then((a) =>
        a.keysGetBackup(orgId, serverId),
    );
    if (!encryptedBackupResp.data)
        throw new Error("No backed up private key found");

    console.log(encryptedBackupResp.data);

    const result = decrypt_backed_up_key_js(
        encryptedBackupResp.data,
        passphrase,
    );
    await privateKeyDb.put({
        _id: result.fingerprint,
        privateKey: result.decrypted_private_pem,
        serverId,
        userId,
        orgId,
    });
}
