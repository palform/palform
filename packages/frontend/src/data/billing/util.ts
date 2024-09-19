import { getOrgSubdomain } from "../auth";
import { frontendURL } from "../common";

export const billingEnabled =
    (import.meta.env.VITE_ENABLE_BILLING as string) === "true";

export const stripeRedirectURL = (orgId: string, orgPath: string) => {
    const orgSubdomain = getOrgSubdomain();
    return new URL(
        `/orgs/${orgId}${orgPath}`,
        orgSubdomain ? `https://${orgSubdomain}.palform.app` : frontendURL
    );
};
