<script lang="ts">
    import { onMount, type Component } from "svelte";
    import OrganisationLoading from "../../layouts/sidebar/OrganisationLoading.svelte";

    interface LazyProps {
        component: Promise<{ default: unknown }>;
        componentProps: Record<string, unknown>;
    }

    let { component, componentProps }: LazyProps = $props();

    let Loaded = $state<Component | undefined>(undefined);
    onMount(() => {
        component.then((c) => {
            Loaded = c.default as Component;
        });
    });
</script>

{#if !Loaded}
    <OrganisationLoading />
{:else}
    <Loaded {...componentProps} />
{/if}
