import { OrganisationMemberRoleEnum } from "@paltiverse/palform-typescript-openapi";

export function orgMemberSelectItems() {
    return Object.values(OrganisationMemberRoleEnum).map((v) => ({
        name: v,
        value: v,
    }));
}
