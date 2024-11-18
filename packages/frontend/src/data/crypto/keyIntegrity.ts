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

// If there are no fingerprints specified in the URL, return undefined
export function getFingerprintsFromURL(url: URL) {
    const urlHash = url.hash.split("#");
    if (urlHash.length !== 2) return undefined;
    const fingerprintListString = new URLSearchParams(urlHash[1]).get("ak");
    if (!fingerprintListString) return undefined;

    const fingerprintList = fingerprintListString.split(",");
    if (fingerprintList.length === 0) return undefined;

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
