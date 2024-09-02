<script lang="ts">
    import type { APITeamAsset } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs, backendURL } from "../../../data/common";
    import { createEventDispatcher, onMount } from "svelte";
    import { getCredentials } from "../../../data/auth";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { DateTime } from "luxon";
    import { Alert, Button, Modal } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";

    export let teamId: string;
    export let allowClear = false;
    export let highlight: string | undefined = undefined;
    export let show: boolean;

    const dispatch = createEventDispatcher<{ select: string | null }>();
    const orgCtx = getOrgContext();

    let assets: APITeamAsset[] | undefined = undefined;
    let assetsLoading = true;
    $: loadAssets = async () => {
        assetsLoading = true;
        const resp = await APIs.teamAssets().then((a) =>
            a.organisationTeamAssetList($orgCtx.org.id, teamId)
        );
        assets = resp.data;
        assetsLoading = false;
    };

    onMount(() => {
        loadAssets();
    });

    let fileInput: HTMLInputElement | undefined;
    let files: FileList | undefined;
    $: onUploadClick = () => {
        if (!fileInput) return;
        fileInput.click();
    };

    let uploadLoading = false;
    $: onFileSet = async () => {
        if (!assets || !files || files.length !== 1) return;
        const file = files.item(0);
        if (!file) return;

        uploadLoading = true;
        try {
            // OpenAPI macros on the backend don't yet support multipart uploads, so we have to do this manually :(
            const fd = new FormData();
            fd.append(file.name, file);
            const auth = await getCredentials();
            if (!auth) return;
            const resp = await fetch(
                backendURL +
                    `/users/me/orgs/${$orgCtx.org.id}/teams/${teamId}/assets`,
                {
                    method: "POST",
                    body: fd,
                    headers: {
                        Authorization:
                            "Basic " +
                            btoa(auth.username + ":" + auth.password),
                    },
                }
            );
            if (resp.status !== 200) {
                throw new Error(await resp.text());
            }

            const newId = await resp.json();

            await showSuccessToast("File uploaded");
            dispatch("select", newId);
            assets = [
                ...assets,
                {
                    id: newId,
                    created_at: DateTime.now().toISO(),
                    url: "",
                },
            ];
        } catch (e) {
            await showFailureToast(e);
        }

        uploadLoading = false;
    };

    $: onAssetSelect = (id: string) => {
        if (uploadLoading) return;
        dispatch("select", id);
    };

    const clearSelection = () => {
        dispatch("select", null);
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
            on:change={onFileSet}
        />
        <div class="flex items-center gap-4">
            <label for="file-upload">
                <LoadingButton
                    on:click={onUploadClick}
                    disabled={uploadLoading}
                    loading={uploadLoading}
                >
                    Upload asset
                </LoadingButton>
            </label>
            {#if allowClear}
                <Button outline on:click={clearSelection}>
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
                    on:click={() => onAssetSelect(asset.id)}
                    on:keypress={() => onAssetSelect(asset.id)}
                    role="button"
                    tabindex={index}
                >
                    <img src={asset.url} alt="Asset" class="w-full" />
                </div>
            {/each}
        </div>
    </Modal>
{/if}
