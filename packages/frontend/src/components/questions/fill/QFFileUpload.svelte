<script lang="ts">
    import type { QuestionSubmissionData } from "@paltiverse/palform-client-js-extra-types/QuestionSubmissionData";
    import type { APIQuestionConfigurationOneOf6 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import {
        fillSendStore,
        formFillStore,
        setQuestionValue,
        sGetFileUpload,
    } from "../../../data/contexts/fill";
    import { Dropzone, Spinner } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faCheckCircle,
        faCloudArrowUp,
    } from "@fortawesome/free-solid-svg-icons";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../../data/contexts/brand";
    import { backendURL } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import { encryptSubmissionAsset } from "../../../data/crypto/submissions";
    import TextButton from "../../TextButton.svelte";

    const brandCtx = getBrandCtx();
    const dispatch = createEventDispatcher<{ change: undefined }>();

    export let id: string;
    export let config: APIQuestionConfigurationOneOf6;
    export let currentValue: QuestionSubmissionData | undefined;
    $: value = currentValue ? sGetFileUpload(currentValue) : { file_id: "" };
    $: allowedTypeCount = config.file_upload.allowed_types.length;

    let uploading = false;
    $: uploadFile = async (file: File) => {
        if (!$formFillStore) return;

        uploading = true;
        setQuestionValue(id, {
            FileUpload: {
                file_id: "",
                content_type: "",
            },
        });

        try {
            const encryptedAsset = await encryptSubmissionAsset(
                file,
                $formFillStore.organisationId,
                $formFillStore.form.f.id,
                $formFillStore.fillAccessToken
            );

            const fd = new FormData();
            fd.append("encrypted", new Blob([encryptedAsset]));
            const resp = await fetch(
                backendURL +
                    `/fill/orgs/${$formFillStore.organisationId}/forms/${$formFillStore.form.f.id}/assets?f=${$formFillStore.fillAccessToken}`,
                {
                    method: "POST",
                    body: fd,
                }
            ).then((e) => e.json() as Promise<string>);

            setQuestionValue(id, {
                FileUpload: {
                    file_id: resp,
                    content_type: file.type,
                },
            });
        } catch (e) {
            await showFailureToast(e);
        }

        dispatch("change");
        uploading = false;
    };

    $: onDrop = async (e: DragEvent) => {
        if (e.dataTransfer?.files) {
            if (e.dataTransfer.files.length !== 1) {
                await showFailureToast("Please drop exactly one file");
                await uploadFile(e.dataTransfer.files.item(0)!);
            }
        }
    };
    $: onChange = async (e: Event) => {
        const t = e.target as HTMLInputElement;
        if (!t.files) return;
        if (t.files.length !== 1) {
            await showFailureToast("Please select exactly one file");
        }

        await uploadFile(t.files.item(0)!);
    };
    $: onClear = (e: Event) => {
        e.stopPropagation();

        setQuestionValue(id, {
            FileUpload: {
                file_id: "",
                content_type: "",
            },
        });
        dispatch("change");
    };

    let accept = "";
    $: {
        accept = "";
        if (!config.file_upload.allowed_types.includes("Other")) {
            for (const type of config.file_upload.allowed_types) {
                switch (type) {
                    case "Image":
                        accept += "image/*,";
                        break;
                    case "Video":
                        accept += "video/*,";
                        break;
                    case "Document":
                        accept +=
                            "application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document,";
                        break;
                    case "Slideshow":
                        accept +=
                            "application/ms-powerpoint,application/vnd.openxmlformats-officedocument.presentationml.presentation";
                        break;
                    case "Spreadsheet":
                        accept +=
                            "application/ms-excel,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
                        break;
                }
            }
        }
    }
</script>

<div
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
    class="overflow-hidden border-gray-300 dark:border-gray-600 border-2 border-dashed"
>
    <Dropzone
        class="h-32 rounded-none border-0"
        on:drop={onDrop}
        on:dragover={(e) => e.preventDefault()}
        on:change={onChange}
        disabled={uploading || $fillSendStore?.loading}
        {accept}
    >
        {#if uploading}
            <Spinner />
            <p class="text-gray-500 dark:text-gray-300 text-sm mt-2">
                Uploading, please wait...
            </p>
        {:else if value.file_id}
            <FontAwesomeIcon
                icon={faCheckCircle}
                class="text-green-400 text-3xl mb-2"
            />
            <p class="text-gray-500 dark:text-gray-300 text-sm">
                File uploaded! Click or drag and drop to change.
            </p>

            <p class="mt-1">
                <TextButton
                    on:click={onClear}
                    disabled={$fillSendStore?.loading}
                >
                    Clear
                </TextButton>
            </p>
        {:else}
            <FontAwesomeIcon
                icon={faCloudArrowUp}
                class="text-gray-400 text-3xl mb-2"
            />
            <p class="text-gray-500 dark:text-gray-300 text-sm mb-1">
                <strong>Click to upload</strong> or drag and drop
            </p>
            <p class="text-gray-500 dark:text-gray-300 text-sm">
                {#if config.file_upload.allowed_types.includes("Any")}
                    All files accepted
                {:else}
                    {#each config.file_upload.allowed_types as fileType, index}
                        {fileType}s{#if index !== allowedTypeCount - 1},&nbsp;{/if}
                    {/each}
                {/if}
                (max 10GB)
            </p>
        {/if}
    </Dropzone>
</div>
