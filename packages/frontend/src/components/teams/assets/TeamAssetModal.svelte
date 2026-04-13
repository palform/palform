<script lang="ts">
    import type { APITeamAsset } from "@palform/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { onMount } from "svelte";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { Alert, Button, Modal } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";

    interface Props {
        teamId: string;
        allowClear?: boolean;
        highlight?: string | undefined;
        show: boolean;
        onselect: (id: string | null) => void;
    }

    let {
        teamId,
        allowClear = false,
        highlight = undefined,
        show = $bindable(),
        onselect,
    }: Props = $props();

    const orgCtx = getOrgContext();

    let assets: APITeamAsset[] | undefined = $state(undefined);
    let assetsLoading = $state(true);
    let loadAssets = $derived(async () => {
        assetsLoading = true;
        const resp = await APIs.teamAssets().then((a) =>
            a.organisationTeamAssetList($orgCtx.org.id, teamId)
        );
        assets = resp.data;
        assetsLoading = false;
    });

    onMount(() => {
        loadAssets();
    });

    let fileInput: HTMLInputElement | undefined = $state();
    let files: FileList | undefined = $state();
    let onUploadClick = $derived(() => {
        if (!fileInput) return;
        fileInput.click();
    });

    let uploadLoading = $state(false);
    let onFileSet = $derived(async () => {
        if (!assets || !files || files.length !== 1) return;
        const file = files.item(0);
        if (!file) return;

        uploadLoading = true;
        try {
            const fd = new FormData();
            fd.append("file", file);

            const teamAssetsApi = await APIs.teamAssets();
            const resp = await teamAssetsApi.organisationTeamAssetUpload(
                $orgCtx.org.id,
                teamId,
                {
                    data: fd,
                }
            );

            await showSuccessToast("File uploaded");
            onselect(resp.data.id);
            assets = [...assets, resp.data];
        } catch (e) {
            await showFailureToast(e);
        }

        uploadLoading = false;
    });

    let onAssetSelect = $derived((id: string) => {
        if (uploadLoading) return;
        onselect(id);
    });

    const clearSelection = () => {
        onselect(null);
    };
</script>

{#if !assetsLoading && assets !== undefined}
    <Modal bind:open={show} outsideclose title="Select asset">
        <input
            type="file"
            class="sr-only"
            id="file-upload"
            bind:this={fileInput}
            bind:files
            onchange={onFileSet}
        />
        <div class="flex items-center gap-4">
            <label for="file-upload">
                <LoadingButton
                    onclick={onUploadClick}
                    disabled={uploadLoading}
                    loading={uploadLoading}
                >
                    Upload asset
                </LoadingButton>
            </label>
            {#if allowClear}
                <Button outline onclick={clearSelection}>
                    Clear selection
                </Button>
            {/if}
        </div>

        {#if assets.length === 0}
            <Alert>
                Your team doesn't have any assets yet. They'll be shown here
                once you upload some.
            </Alert>
        {/if}
        <div class="grid grid-cols-3 gap-4">
            {#each assets as asset, index (asset.id)}
                <div
                    class={`h-40 rounded-lg shadow-md border flex items-center justify-center overflow-hidden ${asset.id === highlight ? "border-4 border-primary-500 cursor-pointer" : ""}`}
                    onclick={() => onAssetSelect(asset.id)}
                    onkeypress={() => onAssetSelect(asset.id)}
                    role="button"
                    tabindex={index}
                >
                    <img src={asset.url} alt="Asset" class="w-full" />
                </div>
            {/each}
        </div>
    </Modal>
{/if}
