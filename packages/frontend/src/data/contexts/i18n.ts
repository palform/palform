import { get, writable } from "svelte/store";

import packEN from "../../i18n/en.json";
import packDE from "../../i18n/de.json";

const I18NLocale: { [key: string]: { [key: string]: string } } = {
    en: packEN,
    de: packDE,
};

export interface I18NContext {
    localeName: string;
}

export const i18nStore = writable<I18NContext>({
    localeName: getBrowserLocale(),
});

function getBrowserLocale(): string {
    if (navigator.languages) {
        const l = navigator.languages[0];
        if (l.startsWith("en")) {
            return "en";
        } else if (l.startsWith("de")) {
            return "de";
        }
    }

    return "en";
}

export function t(label: string) {
    const ctx = get(i18nStore);
    const l = I18NLocale[ctx.localeName];
    if (!l) return label;
    return l[label] ?? label;
}

export function i18nAcceptLanguage() {
    const ctx = get(i18nStore);
    return ctx.localeName ?? "en";
}
