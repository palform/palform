import type { SocialAuthService } from "@paltiverse/palform-typescript-openapi";

export interface OIDCVariables {
    state: string;
    nonce: string;
}

export type OIDCVariablesCategory = "org" | SocialAuthService;

export function saveOIDCVariables(
    category: OIDCVariablesCategory,
    variables: OIDCVariables
) {
    sessionStorage.setItem(`oidc_${category}`, JSON.stringify(variables));
}
export function getOIDCVariables(category: OIDCVariablesCategory) {
    const item = sessionStorage.getItem(`oidc_${category}`);
    if (item === null) return null;
    return JSON.parse(item) as OIDCVariables;
}
export function clearOIDCVariables(category: OIDCVariablesCategory) {
    sessionStorage.removeItem(`oidc_${category}`);
}
