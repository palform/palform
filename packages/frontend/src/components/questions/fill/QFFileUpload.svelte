<script lang="ts">
    import type { ConfigFileUpload } from "@palform/palform-typescript-openapi";
    import {
        fillSendStore,
        formFillStore,
        setQuestionValue,
        sGetFileUpload,
        type QuestionFillProps,
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
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import { encryptSubmissionAsset } from "../../../data/crypto/submissions";
    import { t } from "../../../data/contexts/i18n";
    import QfClearButton from "./QFClearButton.svelte";

    const brandCtx = getBrandCtx();

    interface Props extends QuestionFillProps<ConfigFileUpload> {}

    let { id, config, currentValue, onchange }: Props = $props();
    let value = $derived(
        currentValue ? sGetFileUpload(currentValue) : { file_id: "" }
    );
    let allowedTypeCount = $derived(config.file_upload.allowed_types.length);

    let uploading = $state(false);
    let uploadFile = $derived(async (file: File) => {
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
            const resp = await APIs.fill(
                $formFillStore.fillAccessToken
            ).submissionAssets.submissionAssetsUpload(
                $formFillStore.form.f.id,
                $formFillStore.organisationId,
                {
                    data: fd,
                }
            );

            setQuestionValue(id, {
                FileUpload: {
                    file_id: resp.data.file_id,
                    content_type: file.type,
                },
            });
        } catch (e) {
            await showFailureToast(e);
        }

        onchange();
        uploading = false;
    });

    let onDrop = $derived(async (e: DragEvent) => {
        if (e.dataTransfer?.files) {
            if (e.dataTransfer.files.length !== 1) {
                await showFailureToast("Please drop exactly one file");
                await uploadFile(e.dataTransfer.files.item(0)!);
            }
        }
    });
    let onChange = $derived(async (e: Event) => {
        const t = e.target as HTMLInputElement;
        if (!t.files) return;
        if (t.files.length !== 1) {
            await showFailureToast("Please select exactly one file");
        }

        await uploadFile(t.files.item(0)!);
    });
    let onClear = $derived((e: Event) => {
        e.stopPropagation();

        setQuestionValue(id, {
            FileUpload: {
                file_id: "",
                content_type: "",
            },
        });
        onchange();
    });

    const accept = $derived(() => {
        let _accept = "";
        for (const type of config.file_upload.allowed_types) {
            switch (type) {
                case "Image":
                    _accept += "image/*,";
                    break;
                case "Video":
                    _accept += "video/*,";
                    break;
                case "Document":
                    _accept +=
                        "application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document,";
                    break;
                case "Slideshow":
                    _accept +=
                        "application/ms-powerpoint,application/vnd.openxmlformats-officedocument.presentationml.presentation";
                    break;
                case "Spreadsheet":
                    _accept +=
                        "application/ms-excel,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
                    break;
            }
        }
        return _accept;
    });
</script>

<div
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
    class="overflow-hidden border-gray-300 dark:border-gray-600 border-2 border-dashed"
>
    <Dropzone
        class="h-32 rounded-none border-0"
        {onDrop}
        onDragOver={(e) => e.preventDefault()}
        {onChange}
        disabled={uploading || $fillSendStore?.loading}
        accept={accept()}
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
                    onclick={onClear}
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
