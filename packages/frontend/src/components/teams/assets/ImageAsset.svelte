<script lang="ts">
    import { APIs, backendURL } from "../../../data/common";
    import { formFillStore } from "../../../data/contexts/fill";
    import { Body } from "svelte-body";
    import { getOrgContext } from "../../../data/contexts/orgLayout";

    export let id: string;
    export let teamId: string | undefined = undefined;
    export let height: string | undefined = undefined;
    export let width: string | undefined = undefined;
    export let alt: string | undefined = undefined;
    export let asBodyBackground = false;

    const orgCtx = getOrgContext();

    $: assetUrl = $formFillStore
        ? backendURL +
          `/fill/orgs/${$formFillStore.organisationId}/forms/${$formFillStore.form.f.id}/assets/${id}?f=${$formFillStore.fillAccessToken}`
        : undefined;

    let resolvedAssetUrl: string | undefined = undefined;

    $: (async () => {
        if (assetUrl === undefined && teamId !== undefined) {
            resolvedAssetUrl = undefined;
            const resp = await APIs.teamAssets().then((a) =>
                a.organisationTeamAssetGet($orgCtx.org.id, teamId, id)
            );
            resolvedAssetUrl = resp.data;
        }
    })();
</script>

{#if asBodyBackground}
    {#if assetUrl}
        <Body
            style={{
                backgroundImage: `url(${assetUrl})`,
            }}
            class="bg-center bg-cover bg-repeat bg-fixed backdrop-brightness-50"
        />
    {/if}
{:else if assetUrl || resolvedAssetUrl}
    <img
        src={assetUrl ?? resolvedAssetUrl}
        style:height
        style:width
        class={$$props.class}
        {alt}
    />
{/if}
