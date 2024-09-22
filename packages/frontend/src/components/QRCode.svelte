<script lang="ts">
    import { faDownload } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button } from "flowbite-svelte";
    import QRCode from "qrcode";

    export let uri: string;
    export let download = false;
    let canvas: HTMLCanvasElement | undefined;

    $: {
        if (canvas) {
            QRCode.toCanvas(canvas, uri);
        }
    }

    $: onDownloadClick = async () => {
        if (!canvas) return;

        const link = document.createElement("a");
        link.href = canvas.toDataURL();
        link.download = "qr_code.png";
        link.click();
    };
</script>

<div class={$$props.class}>
    <canvas bind:this={canvas} />
    {#if download}
        <Button color="light" size="sm" class="mt-2" on:click={onDownloadClick}>
            <FontAwesomeIcon icon={faDownload} class="me-2" />
            Download PNG
        </Button>
    {/if}
</div>
