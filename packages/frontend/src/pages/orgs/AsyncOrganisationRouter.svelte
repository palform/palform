<script lang="ts">
    import LazyComponent from "../../components/lazy/LazyComponent.svelte";
    const OrganisationRouter = import("./OrganisationRouter.svelte");

    export let orgId: string;
</script>

<LazyComponent component={OrganisationRouter} props={{ orgId }} />
