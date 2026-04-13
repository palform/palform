<script lang="ts">
    import { TabItem } from "flowbite-svelte";
    import Redirect from "../Redirect.svelte";
    import { route } from "../../router";

    interface Props {
        title: string;
        path: string;
        disabled?: boolean;
    }

    let { title, path, disabled = false }: Props = $props();

    let pathComponents = $derived(route.pathname.split("/").slice(1) ?? []);
    let active = $derived(() => {
        if (pathComponents.length === 0) return false;
        return pathComponents[pathComponents.length - 1] === path;
    });

    let constructPath = $derived(() => {
        return ["", ...pathComponents.slice(0, -1), path].join("/");
    });
</script>

<TabItem {title} open={active()} {disabled}>
    <Redirect href={constructPath()} />
</TabItem>
