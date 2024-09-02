import { faLink } from "@fortawesome/free-solid-svg-icons";
import { showToast } from "../toast";

export async function copyGenericValue(url: string) {
    await navigator.clipboard.writeText(url);
    await showToast({
        label: "Copied to clipboard!",
        icon: faLink,
    });
}
