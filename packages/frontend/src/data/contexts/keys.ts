import { writable } from "svelte/store";
import { privateKeyDb } from "../pouch";
import type { KeyResolver } from "@paltiverse/palform-crypto";

export const privateKeyStore = writable<KeyResolver>();

export async function getPrivateKeys() {
    const keys = await privateKeyDb
        .allDocs({ include_docs: true })
        .then((resp) =>
            resp.rows
                .map((e) => e.doc!)
                .filter((e) => typeof e.privateKey === "string")
        );
    return keys.map((e) => e.privateKey);
}
