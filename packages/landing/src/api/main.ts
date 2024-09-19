import {
    BillingPlansApi,
    Configuration,
} from "@paltiverse/palform-typescript-openapi/src/index";

const apiConfig = new Configuration({
    basePath: import.meta.env.PUBLIC_BACKEND_URL,
});

export const billingAPI = new BillingPlansApi(apiConfig);
