<script lang="ts">
    import { faDownload } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button } from "flowbite-svelte";
    import QRCode from "qrcode";

    interface Props {
        uri: string;
        download?: boolean;
        class?: string;
    }

    let { uri, download = false, class: className }: Props = $props();
    let canvas: HTMLCanvasElement | undefined = $state(undefined);

    $effect(() => {
        if (canvas) {
            QRCode.toCanvas(canvas, uri);
        }
    });

    async function onDownloadClick() {
        if (!canvas) return;

        const link = document.createElement("a");
        link.href = canvas.toDataURL();
        link.download = "qr_code.png";
        link.click();
    }
</script>

<div class={className}>
    <canvas bind:this={canvas}></canvas>
    {#if download}
        <Button color="light" size="sm" class="mt-2" onclick={onDownloadClick}>
            <FontAwesomeIcon icon={faDownload} class="me-2" />
            Download PNG
        </Button>
    {/if}
</div>
