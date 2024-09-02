<script lang="ts">
    import { TabItem } from "flowbite-svelte";
    import Redirect from "../Redirect.svelte";
    import { useLocation } from "svelte-routing";

    export let title: string;
    export let path: string;
    export let disabled = false;

    const location = useLocation();
    $: pathComponents = $location.pathname.split("/").slice(1) ?? [];
    $: active = () => {
        if (pathComponents.length === 0) return false;
        return pathComponents[pathComponents.length - 1] === path;
    };

    $: constructPath = () => {
        return ["", ...pathComponents.slice(0, -1), path].join("/");
    };
</script>

<TabItem {title} open={active()} {disabled}>
    <Redirect href={constructPath()} />
</TabItem>
