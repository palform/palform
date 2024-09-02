import { frontendURL } from "./common";
import { copyGenericURL } from "./util/clipboard";

export function formatOrgInviteLink(orgId: string, inviteId: string) {
    return new URL(
        `/orgs/join/${orgId}/${inviteId}`,
        frontendURL,
    ).toString();
}

export async function copyOrgInviteLink(orgId: string, inviteId: string) {
    await copyGenericURL(formatOrgInviteLink(orgId, inviteId));
}
