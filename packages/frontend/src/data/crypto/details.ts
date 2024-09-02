import type {
    DecrpytingKey,
    DecrpytingKeyOneOf,
} from "@paltiverse/palform-typescript-openapi";

export function isKnownKey(key: DecrpytingKey): key is DecrpytingKeyOneOf {
    return "Known" in key;
}
