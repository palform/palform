import { DateTime } from "luxon";
import { APIs, urlsBase } from "../common";
import { authDb } from "../pouch";
import type { BaseAPI } from "@paltiverse/palform-typescript-openapi/src/base";
import {
    Configuration,
    type ConfigurationParameters,
    type NewAPIAuthToken,
} from "@paltiverse/palform-typescript-openapi";
import { navigate } from "svelte-routing";
import { showFailureToast } from "../toast";
import { AxiosError } from "axios";
import {
    clearOIDCVariables,
    getOIDCVariables,
    saveOIDCVariables,
} from "./variables";

async function getActiveAuthentication() {
    const activeAuths = await authDb.find({
        selector: {
            expires: {
                $gte: DateTime.now().toMillis(),
            },
        },
        sort: [{ expires: "desc" }],
        limit: 1,
    });

    if (activeAuths.docs.length === 0) return undefined;
    return activeAuths.docs[activeAuths.docs.length - 1];
}

export async function getCredentials() {
    const auth = await getActiveAuthentication();
    if (!auth) return;
    return {
        username: auth._id,
        password: auth.tokenSecret,
    };
}

export async function isSignedIn() {
    return (await getActiveAuthentication()) !== undefined;
}

function redirectURL(subdomain: string) {
    const url = new URL("/auth/callback", urlsBase);
    if (window.location.hostname !== "localhost") {
        url.hostname = subdomain + "." + url.hostname;
    }
    return url.toString();
}

export function getOrgSubdomain() {
    const url = window.location.hostname.split(".");
    if (window.location.hostname.endsWith("localhost")) {
        if (url.length < 2) return null;
    } else {
        if (url.length < 3) return null;
    }

    const subdomain = url[0];
    if (subdomain === "dash") return null;
    return subdomain;
}

export async function getOrgSubdomainIDForAuth(subdomain: string | null) {
    if (subdomain === null) return null;
    try {
        const resp = await APIs.orgsNoAuth.orgsResolveSubdomain(subdomain);
        return resp.data;
    } catch (e) {
        if (e instanceof AxiosError && e.response?.status === 404) {
            return null;
        }
        throw e;
    }
}

export async function getSignInURL() {
    const subdomain = getOrgSubdomain();
    if (!subdomain) return;
    const orgId = await getOrgSubdomainIDForAuth(subdomain);
    if (!orgId) return;

    const response = await APIs.auth.authStart(orgId, redirectURL(subdomain));
    saveOIDCVariables("org", {
        state: response.data.state,
        nonce: response.data.nonce,
    });
    return response.data.url;
}

export async function performAuthCallback(
    orgId: string,
    code: string,
    state: string
) {
    const vars = getOIDCVariables("org");
    if (!vars) throw new Error("Session missing; please restart signing in");
    const subdomain = getOrgSubdomain();
    if (!subdomain) throw new Error("Could not find organisation");
    clearOIDCVariables("org");

    if (state !== vars.state)
        throw new Error("State does not match! Please restart signing in");

    const response = await APIs.auth.authCallback(orgId, {
        auth_code: code,
        nonce: vars.nonce,
        redirect_url: redirectURL(subdomain),
    });
    await saveAuthToken(response.data.token);
}

export async function saveAuthToken(token: NewAPIAuthToken) {
    await authDb.put({
        _id: token.id,
        tokenSecret: token.secret,
        created: DateTime.now().toMillis(),
        expires: DateTime.fromISO(token.expires_at).toMillis(),
    });
}

export async function signInWithEmailAndPassword(
    email: string,
    password: string,
    captcha: string,
    createInitialOrg: boolean
): Promise<{
    newOrgId?: string;
    tfaSessionId?: string;
}> {
    const response = await APIs.auth.authSignIn(captcha, {
        Credentials: {
            email,
            password,
            create_initial_org: createInitialOrg,
        },
    });

    if ("Done" in response.data) {
        await saveAuthToken(response.data.Done.token);
        return { newOrgId: response.data.Done.new_org_id ?? undefined };
    } else if ("SecondFactorRequired" in response.data) {
        return {
            newOrgId:
                response.data.SecondFactorRequired.new_org_id ?? undefined,
            tfaSessionId: response.data.SecondFactorRequired.session_id,
        };
    } else {
        throw new Error("Unrecognised response");
    }
}

export async function signOut() {
    const auth = await getActiveAuthentication();
    if (!auth) return;
    await APIs.authWithToken().then((a) => a.authInvalidateToken());
    await authDb.remove(auth);
}

export class UnauthenticatedError extends Error {
    message = "Not signed in!";
}

export async function apiWithAuth<T extends BaseAPI>(
    api: {
        new (configuration: Configuration): T;
    },
    configuration: ConfigurationParameters
): Promise<T> {
    const auth = await getCredentials();
    if (!auth) {
        await showFailureToast("Please sign in to continue");
        navigate("/auth/login");
        throw new UnauthenticatedError();
    }

    return new api(new Configuration({ ...auth, ...configuration }));
}

export async function getUserId() {
    const auth = await getCredentials();
    if (!auth) return;
    const resp = await APIs.auth.authTest({
        auth,
    });
    return resp.data.user.id;
}
