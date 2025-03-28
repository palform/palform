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
    import { t } from "../../../data/contexts/i18n";
    import QfClearButton from "./QFClearButton.svelte";

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
                $formFillStore.fillAccessToken,
                !$formFillStore.isShortLink
            );

            if (encryptedAsset.byteLength > 10 * 1e9) {
                throw t("file_too_large");
            }

            const fd = new FormData();
            fd.append("encrypted", new Blob([encryptedAsset]));
            const resp = await fetch(
                backendURL +
                    `/fill/orgs/${$formFillStore.organisationId}/forms/${$formFillStore.form.f.id}/assets?f=${$formFillStore.fillAccessToken}`,
                {
                    method: "POST",
                    body: fd,
                }
            );

            const respJson = (await resp.json()) as string;

            if (!resp.ok) {
                throw respJson;
            }

            setQuestionValue(id, {
                FileUpload: {
                    file_id: respJson,
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
                {t("file_uploading")}
            </p>
        {:else if value.file_id}
            <FontAwesomeIcon
                icon={faCheckCircle}
                class="text-green-400 text-3xl mb-2"
            />
            <p class="text-gray-500 dark:text-gray-300 text-sm">
                {t("file_uploaded")}
            </p>

            <p class="mt-1">
                <QfClearButton
                    disabled={$fillSendStore?.loading}
                    on:click={onClear}
                />
            </p>
        {:else}
            <FontAwesomeIcon
                icon={faCloudArrowUp}
                class="text-gray-400 text-3xl mb-2"
            />
            <p class="text-gray-500 dark:text-gray-300 text-sm mb-1">
                <strong>{t("file_upload_1")}</strong>{t("file_upload_2")}
            </p>
            <p class="text-gray-500 dark:text-gray-300 text-sm">
                {#if config.file_upload.allowed_types.includes("Any")}
                    {t("file_accepted_all")}
                {:else}
                    {#each config.file_upload.allowed_types as fileType, index}
                        {fileType}s{#if index !== allowedTypeCount - 1},&nbsp;{/if}
                    {/each}
                {/if}
                ({t("file_max_size")})
            </p>
        {/if}
    </Dropzone>
</div>
