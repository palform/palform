export function isOrgRouteMatch(
    currentPath: string,
    orgRoute: string,
    activationLevel: number
) {
    const currentPathComponents = currentPath.split("/").slice(3);
    const orgRouteComponents = orgRoute.split("/").slice(1);
    if (orgRouteComponents.length < activationLevel) {
        throw new Error(
            `Provided route had ${orgRouteComponents.length} components, but activation level was set to ${activationLevel}`
        );
    }

    for (let i = 0; i < activationLevel; i++) {
        if (i >= currentPathComponents.length) {
            return false;
        }

        if (currentPathComponents[i] !== orgRouteComponents[i]) {
            return false;
        }
    }

    return true;
}
