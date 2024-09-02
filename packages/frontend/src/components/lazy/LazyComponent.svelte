<script lang="ts" generics="Props extends Record<string, any>">
    import { onMount, type SvelteComponent } from "svelte";
    import OrganisationLoading from "../../layouts/sidebar/OrganisationLoading.svelte";

    type AsyncSvelteComponent = Promise<{
        default: typeof SvelteComponent<Props>;
    }>;
    export let component: AsyncSvelteComponent;
    export let props: Props;

    let loadedComponent: typeof SvelteComponent<Props> | undefined;
    onMount(() => {
        component.then((c) => {
            loadedComponent = c.default;
        });
    });
</script>

{#if !loadedComponent}
    <OrganisationLoading />
{/if}

{#if loadedComponent}
    <svelte:component this={loadedComponent} {...props} />
{/if}
