import type {
    APIFormTemplate,
    APIFormTemplateCategory,
} from "@paltiverse/palform-typescript-openapi";
import { getContext, setContext } from "svelte";
import type { Writable } from "svelte/store";

export interface TemplatesContext {
    categories: APIFormTemplateCategory[];
    templates: APIFormTemplate[];
}
export const setTemplatesContext = (ctx: Writable<TemplatesContext>) => {
    setContext("new_form_templates", ctx);
};
export const getTemplatesContext = () => {
    return getContext<Writable<TemplatesContext>>("new_form_templates");
};
