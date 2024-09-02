import { derived, readable } from "svelte/store";
import { getOrgContext } from "../contexts/orgLayout";
import type { APIEntitlementInfo } from "@paltiverse/palform-typescript-openapi";

export function isEntitled(
    key: keyof APIEntitlementInfo,
    requireMulti = false
) {
    return derived([readable(key), getOrgContext()], ([$key, $orgCtx]) => {
        if ($orgCtx.entitlements === undefined) return true;
        const val = $orgCtx.entitlements[$key];
        if (val === null || val === undefined) return true;
        if (typeof val === "number") {
            if (val > 1) return true;
            if (val === 1 && !requireMulti) return true;
            return false;
        }
        return val;
    });
}
