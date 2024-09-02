import { frontendURL, urlsBase } from "./common";
import { copyGenericURL } from "./util/clipboard";

export function formatFillTokenURL(
    orgId: string,
    formId: string,
    tokenId: string
) {
    return new URL(
        `/fill/${orgId}/${formId}?f=${tokenId}`,
        frontendURL
    ).toString();
}

export function formatShortLinkURL(subdomain: string, shortLink: string) {
    const url = new URL(`/${shortLink}`, urlsBase);
    url.hostname = subdomain + "." + url.hostname;
    return url.toString();
}

export async function copyFillToken(
    orgId: string,
    formId: string,
    tokenId: string
) {
    await copyGenericURL(formatFillTokenURL(orgId, formId, tokenId));
}
