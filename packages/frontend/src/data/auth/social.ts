import type { SocialAuthService } from "@paltiverse/palform-typescript-openapi";
import googleLogo from "../../assets/social_auth/google.png";
import { APIs, frontendURL } from "../common";
import {
    clearOIDCVariables,
    getOIDCVariables,
    saveOIDCVariables,
} from "./variables";
import { saveAuthToken } from ".";

export function isSocialAuthProvider(name: string): name is SocialAuthService {
    return ["Google"].includes(name);
}

export function socialAuthServiceName(service: SocialAuthService) {
    switch (service) {
        case "Google":
            return "Google";
    }
}

export function socialAuthServiceIcon(service: SocialAuthService) {
    switch (service) {
        case "Google":
            return googleLogo;
    }
}

function socialAuthRedirectURL(service: SocialAuthService) {
    return new URL(`/auth/social/${service}/callback`, frontendURL);
}

export async function startSocialAuth(service: SocialAuthService) {
    const response = await APIs.auth.authSocialStart({
        redirect_url: socialAuthRedirectURL(service).toString(),
        service,
    });

    saveOIDCVariables(service, {
        state: response.data.state,
        nonce: response.data.nonce,
    });

    return response.data.url;
}

export async function processSocialAuthCallback(service: SocialAuthService) {
    const params = new URLSearchParams(window.location.search);
    const code = params.get("code");
    const state = params.get("state");

    if (!code || !state) throw new Error("Code or state not found in URL");

    const storedVariables = getOIDCVariables(service);
    if (!storedVariables)
        throw new Error(
            "State not found in session. Please restart the sign in process."
        );

    if (storedVariables.state !== state)
        throw new Error(
            "State does not mach expected value. Please restart the sign in process."
        );

    clearOIDCVariables(service);
    const response = await APIs.auth.authSocialCallback({
        service,
        code,
        nonce: storedVariables.nonce,
        redirect_url: socialAuthRedirectURL(service).toString(),
    });
    await saveAuthToken(response.data.token);

    return response.data.new_org_id;
}
