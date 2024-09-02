import type { FontAwesomeIconProps } from "@fortawesome/svelte-fontawesome";

export interface PalcrumbPath {
    name: string;
    href?: string;
    icon?: FontAwesomeIconProps["icon"];
}
