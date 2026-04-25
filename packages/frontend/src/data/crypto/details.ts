import type {
    DecrpytingKey,
    Known,
} from "@palform/palform-typescript-openapi";

export function isKnownKey(key: DecrpytingKey): key is Known {
    return "Known" in key;
}
