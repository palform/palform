import type { Key } from "openpgp";
import { APIs } from "../common";

export async function getTeamKeyFingerprints(orgId: string, teamId: string) {
    const fingerprintsResp = await APIs.orgKeys().then((a) =>
        a.orgKeysTeamFingerprints(orgId, teamId)
    );
    return fingerprintsResp.data;
}

export function addFingerprintsToURL(url: URL, fingerprints: string[]) {
    const newURL = new URL(url);
    newURL.hash = `ak=${fingerprints.join(",")}`;
    return newURL;
}

export function getFingerprintsFromURL(url: URL) {
    const urlHash = url.hash.split("#");
    if (urlHash.length !== 2) throw new Error("URL missing key fingerprints");
    const fingerprintListString = new URLSearchParams(urlHash[1]).get("ak");
    if (!fingerprintListString) throw new Error("URL missing key fingerprints");
    const fingerprintList = fingerprintListString.split(",");
    if (fingerprintList.length === 0)
        throw new Error("URL did not contain any key fingerprints");

    return fingerprintList;
}

export function filterKeysByFingerprint(
    keys: Key[],
    acceptedFingerprints: string[]
) {
    return keys.filter((k) => {
        return acceptedFingerprints
            .map((e) => e.toLowerCase())
            .includes(k.getFingerprint().toLowerCase());
    });
}
