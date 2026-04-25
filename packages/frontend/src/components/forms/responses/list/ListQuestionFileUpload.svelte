<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import LoadingButton from "../../../LoadingButton.svelte";
    import { faDownload } from "@fortawesome/free-solid-svg-icons";
    import { APIs } from "../../../../data/common";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import { showFailureToast } from "../../../../data/toast";
    import { getFormAdminContext } from "../../../../data/contexts/formAdmin";
    import { decryptSubmissionAsset } from "../../../../data/crypto/results";

    interface Props {
        fileId: string;
        contentType: string;
        compact: boolean;
        [key: string]: any;
    }

    let { ...props }: Props = $props();
    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();

    let loading = $state(false);
    let onDownloadClick = $derived(async () => {
        loading = true;

        try {
            const resp = await APIs.submissionAssets().then((a) =>
                a.submissionAssetsGetLink(
                    $orgCtx.org.id,
                    $formAdminCtx.formId,
                    props.fileId
                )
            );

            const decryptedData = await decryptSubmissionAsset(
                // For some reason Axios auto-parses the bytes response into a string but the OpenAPI generator thinks it's still number[]
                resp.data as unknown as string
            );
            const blob = new Blob([decryptedData], { type: props.contentType });

            const link = document.createElement("a");
            link.href = window.URL.createObjectURL(blob);
            link.download = "submission_asset";
            link.click();
        } catch (e) {
            console.error(e);
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

{#if props.fileId !== ""}
    <LoadingButton
        color="light"
        onclick={onDownloadClick}
        disabled={loading}
        size={props.compact ? "xs" : "md"}
        buttonClass={props.class}
        {loading}
    >
        <FontAwesomeIcon icon={faDownload} class="me-2" />
        Download
        {#if !props.compact}
            file{/if}
    </LoadingButton>
{/if}
