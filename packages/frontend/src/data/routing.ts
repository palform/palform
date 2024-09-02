export function isOrgRouteMatch(currentPath: string, orgRoute: string) {
    const strippedPath = `/${currentPath.split("/").slice(3).join("/")}`;
    if (strippedPath === "/") {
        return orgRoute === "/";
    }
    if (orgRoute === "/") return false;
    return strippedPath.startsWith(orgRoute);
}
