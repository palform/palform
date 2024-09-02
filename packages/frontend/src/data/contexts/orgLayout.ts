import type {
    APIEntitlementInfo,
    APIForm,
    APIOrganisation,
    APIOrganisationTeamMembership,
    InductionStatus,
} from "@paltiverse/palform-typescript-openapi";
import { getContext, setContext } from "svelte";
import {
    derived,
    get,
    readable,
    type Readable,
    type Writable,
} from "svelte/store";
import { getResponsesContext } from "./results";
import { APIs } from "../common";
import { orgVisitDb } from "../pouch";
import { DateTime } from "luxon";

export interface GlobalWarningMessage {
    message: string;
    link: string;
    hideOnPaths: string[];
}

export interface OrgContext {
    org: APIOrganisation;
    amIAdmin: boolean;
    forms: APIForm[];
    myTeams: APIOrganisationTeamMembership[];
    induction: InductionStatus;
    entitlements: APIEntitlementInfo | undefined;
    globalWarning?: GlobalWarningMessage;
}

export const setOrgContext = (ctx: Writable<OrgContext>) => {
    setContext("orgCtx", ctx);
};

export const getOrgContext = () => getContext<Writable<OrgContext>>("orgCtx");

export function getFormCtx(formId: string): Readable<APIForm | undefined>;
export function getFormCtx(formId?: undefined): Readable<APIForm>;
export function getFormCtx(formId?: string) {
    return derived(
        [
            formId === undefined ? getResponsesContext() : readable(formId),
            getOrgContext(),
        ],
        ([$d, $ctx]) => {
            if (typeof $d === "string") {
                return $ctx.forms.find((e) => e.id === $d);
            }
            return $ctx.forms.find((e) => e.id === $d.formId);
        }
    );
}
export function updateFormCtx(
    ctx: Writable<OrgContext>,
    formId: string,
    cb: ((update: APIForm) => void) | APIForm
) {
    ctx.update((ctx) => {
        const i = ctx.forms.findIndex((e) => e.id === formId);
        if (i === -1) return ctx;

        if (typeof cb === "function") {
            cb(ctx.forms[i]);
        } else {
            Object.assign(ctx.forms[i], cb);
        }
        return ctx;
    });
}

export async function markOrgViewNow(orgId: string) {
    let currentRev: string | undefined;
    try {
        const current = await orgVisitDb.get(orgId);
        currentRev = current._rev;
    } catch (e) {
        currentRev = undefined;
    }
    await orgVisitDb.put({
        _id: orgId,
        _rev: currentRev,
        time: DateTime.now().toMillis(),
    });
}
export async function getLastOrgView(
    orgId: string
): Promise<DateTime | undefined> {
    try {
        const visit = await orgVisitDb.get(orgId);
        return DateTime.fromMillis(visit.time);
    } catch (e) {
        return undefined;
    }
}

export async function reloadGlobalAlert(ctx: Writable<OrgContext>) {
    const val = get(ctx);
    const resp = await APIs.induction().then((a) =>
        a.inductionAlert(val.org.id)
    );
    ctx.update((ctx) => {
        if (!resp.data) {
            return {
                ...ctx,
                globalWarning: undefined,
            };
        }

        const hideOnPaths = resp.data.hide_on.map((e) => {
            switch (e) {
                case "Induction":
                    return "/induction";
                case "Keys":
                    return "/user/keys";
            }
        });

        if (resp.data.alert_type === "NoActiveKey") {
            ctx.globalWarning = {
                message:
                    "Your account doesn't have an active key. You won't be able to receive form responses until you create one.",
                link: `/orgs/${val.org.id}/user/keys`,
                hideOnPaths,
            };
        }

        return ctx;
    });
}

export async function reloadInduction(ctx: Writable<OrgContext>) {
    const val = get(ctx);
    APIs.induction()
        .then((a) => a.inductionStatus(val.org.id))
        .then((resp) => {
            ctx.update((ctx) => {
                return {
                    ...ctx,
                    induction: resp.data,
                };
            });
        });
}
