<script lang="ts">
    import QRCode from "qrcode";

    export let uri: string;
    let canvas: HTMLCanvasElement | undefined;

    $: {
        if (canvas) {
            QRCode.toCanvas(canvas, uri);
        }
    }
</script>

<canvas bind:this={canvas} />
