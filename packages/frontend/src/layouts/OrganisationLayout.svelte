<script lang="ts">
    import OrganisationSidebar from "./sidebar/OrganisationSidebar.svelte";
    import { route } from "../router";

    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();

    const orgId = $derived(route.params.orgId ?? "");
</script>

<OrganisationSidebar {orgId}>
    {@render children?.()}
</OrganisationSidebar>
