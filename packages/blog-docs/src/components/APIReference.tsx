import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";

const excludedPathPrefixes = [
    "/api/health",
    "/api/auth",
    "/api/countries",
    "/api/feedback",
    "/api/billing",
    "/api/org/{org_id}/billing",
    "/api/captcha",
    "/api/org/{org_id}/induction",
];

type Security = Record<string, string[]>[];
type Operation = { security?: Security };
type OpenAPISpec = {
    security?: Security;
    paths: Record<string, Record<string, Operation>>;
};

// Security is a list of alternatives, any of which is sufficient.
// An empty list, or an empty alternative, means no authentication is needed.
const isPublicSecurity = (security: Security) =>
    security.length === 0 ||
    security.some(
        (requirement) =>
            Object.keys(requirement).length === 0 ||
            "service_account_token" in requirement
    );

const isPathExcluded = (path: string) => {
    return excludedPathPrefixes.some((prefix) => path.startsWith(prefix));
};

// Remove operations that are only available to personal auth tokens
const filterPublicOperations = (spec: OpenAPISpec): OpenAPISpec => ({
    ...spec,
    paths: Object.fromEntries(
        Object.entries(spec.paths).map(([path, operations]) => [
            path,
            Object.fromEntries(
                Object.entries(operations).filter(([, operation]) => {
                    if (isPathExcluded(path)) return false;
                    return isPublicSecurity(
                        operation.security ?? spec.security ?? []
                    );
                })
            ),
        ])
    ),
});

const ReadOnlyPlugin = () => ({
    components: {
        info: () => null,
        authorizeOperationBtn: () => null,
    },
    statePlugins: {
        spec: {
            wrapSelectors: {
                // Without security definitions, the whole Authorize bar is omitted
                securityDefinitions: () => () => null,
            },
        },
    },
});

export default function APIReference() {
    const { siteConfig } = useDocusaurusContext();
    const backendUrl = siteConfig.customFields?.backendUrl as string;
    const [spec, setSpec] = useState<OpenAPISpec>();

    useEffect(() => {
        fetch(`${backendUrl}/openapi.json`)
            .then((response) => response.json())
            .then((spec) => setSpec(filterPublicOperations(spec)));
    }, [backendUrl]);

    if (!spec) return null;

    return (
        <SwaggerUI
            spec={spec}
            plugins={[ReadOnlyPlugin]}
            supportedSubmitMethods={[]}
            defaultModelExpandDepth={3}
            defaultModelRendering="model"
            filter
        />
    );
}
