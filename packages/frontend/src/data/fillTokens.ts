import { frontendURL, urlsBase } from "./common";
import {
    addFingerprintsToURL,
    getTeamKeyFingerprints,
} from "./crypto/keyIntegrity";
import { copyGenericValue } from "./util/clipboard";

export async function formatFillTokenURL(
    orgId: string,
    teamId: string,
    formId: string,
    tokenId: string
) {
    const url = new URL(`/fill/${orgId}/${formId}?f=${tokenId}`, frontendURL);

    const fingerprints = await getTeamKeyFingerprints(orgId, teamId);
    return addFingerprintsToURL(url, fingerprints).toString();
}

export function formatShortLinkURL(subdomain: string, shortLink: string) {
    const url = new URL(`/${shortLink}`, urlsBase);
    url.hostname = subdomain + "." + url.hostname;
    return url.toString();
}

export async function copyFillToken(
    orgId: string,
    teamId: string,
    formId: string,
    tokenId: string
) {
    await copyGenericValue(
        await formatFillTokenURL(orgId, teamId, formId, tokenId)
    );
}
