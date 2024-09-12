<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import LoadingButton from "../../../LoadingButton.svelte";
    import { faDownload } from "@fortawesome/free-solid-svg-icons";
    import { APIs } from "../../../../data/common";
    import { getOrgContext } from "../../../../data/contexts/orgLayout";
    import { decryptSubmissionAsset } from "../../../../data/crypto/submissions";
    import { showFailureToast } from "../../../../data/toast";
    import { getFormAdminContext } from "../../../../data/contexts/formAdmin";

    export let fileId: string;
    export let contentType: string;
    export let compact: boolean;
    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();

    let loading = false;
    $: onDownloadClick = async () => {
        loading = true;

        try {
            const resp = await APIs.submissionAssets().then((a) =>
                a.submissionAssetsGetLink(
                    $orgCtx.org.id,
                    $formAdminCtx.formId,
                    fileId
                )
            );

            const decryptedData = await decryptSubmissionAsset(
                // For some reason Axios auto-parses the bytes response into a string but the OpenAPI generator thinks it's still number[]
                resp.data as unknown as string
            );
            const blob = new Blob([decryptedData], { type: contentType });

            const link = document.createElement("a");
            link.href = window.URL.createObjectURL(blob);
            link.download = "submission_asset";
            link.click();
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

{#if fileId !== ""}
    <LoadingButton
        color="light"
        on:click={onDownloadClick}
        disabled={loading}
        size={compact ? "xs" : "md"}
        buttonClass={$$props.class}
        {loading}
    >
        <FontAwesomeIcon icon={faDownload} class="me-2" />
        Download
        {#if !compact}
            file{/if}
    </LoadingButton>
{/if}
