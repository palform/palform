import type { AuditLogTargetResourceEnum } from "@palform/palform-typescript-openapi";

export interface AuditLogRequestFilters {
    from: string | null;
    to: string | null;
    resource: AuditLogTargetResourceEnum | null;
}
