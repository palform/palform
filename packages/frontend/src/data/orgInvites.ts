import { frontendURL } from "./common";
import { copyGenericValue } from "./util/clipboard";

export function formatOrgInviteLink(orgId: string, inviteId: string) {
    return new URL(
        `/orgs/join/${orgId}/${inviteId}`,
        frontendURL,
    ).toString();
}

export async function copyOrgInviteLink(orgId: string, inviteId: string) {
    await copyGenericValue(formatOrgInviteLink(orgId, inviteId));
}
