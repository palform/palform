<script lang="ts">
    import { APIs, backendURL } from "../../../data/common";
    import { formFillStore } from "../../../data/contexts/fill";
    import { Body } from "svelte-body";

    interface Props {
        id: string;
        orgId?: string;
        teamId?: string;
        height?: string;
        width?: string;
        alt?: string;
        asBodyBackground?: boolean;
        class?: string;
    }

    let {
        id,
        orgId = undefined,
        teamId = undefined,
        height = undefined,
        width = undefined,
        alt = undefined,
        asBodyBackground = false,
        class: className,
    }: Props = $props();

    let assetUrl = $derived(
        $formFillStore
            ? backendURL +
                  `/fill/orgs/${$formFillStore.organisationId}/forms/${$formFillStore.form.f.id}/assets/${id}?f=${$formFillStore.fillAccessToken}`
            : undefined
    );

    let resolvedAssetUrl = $state<string | undefined>(undefined);

    $effect(() => {
        if (assetUrl !== undefined) {
            resolvedAssetUrl = undefined;
            return;
        }
        if (teamId === undefined || orgId === undefined || id === undefined) {
            return;
        }
        resolvedAssetUrl = undefined;
        let cancelled = false;
        APIs.teamAssets()
            .then((a) => a.organisationTeamAssetGet(orgId, teamId, id))
            .then((resp) => {
                if (!cancelled) {
                    resolvedAssetUrl = resp.data;
                }
            });
        return () => {
            cancelled = true;
        };
    });
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
        class={className}
        {alt}
    />
{/if}
