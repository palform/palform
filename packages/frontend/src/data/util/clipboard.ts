import { faLink } from "@fortawesome/free-solid-svg-icons";
import { showToast } from "../toast";

export async function copyGenericURL(url: string) {
    await navigator.clipboard.writeText(url);
    await showToast({
        label: "Copied URL to clipboard!",
        icon: faLink,
    });
}
