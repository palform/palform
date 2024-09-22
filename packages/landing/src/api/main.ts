import {
    BillingPlansApi,
    Configuration,
    FormTemplatesApi,
} from "@paltiverse/palform-typescript-openapi/src/index";

export const APIBaseURL = import.meta.env.PUBLIC_BACKEND_URL as string;
export const AppBaseURL = import.meta.env.PUBLIC_APP_URL as string;

const apiConfig = new Configuration({
    basePath: APIBaseURL,
});

export const billingAPI = new BillingPlansApi(apiConfig);
export const formTemplatesAPI = new FormTemplatesApi(apiConfig);
