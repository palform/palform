<script lang="ts">
    import Color from "colorjs.io";
    import {
        getBaseREMFontSizeForBrand,
        getBrandCtx,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";

    const ctx = getBrandCtx();
    export let textLighter = false;
    export let ignoreColor = false;
    export let sizeGroup: "h1+" | "h1" | "h2" | "p" | undefined = undefined;

    let textColorOverride: string | undefined = undefined;
    let fontSizeOverride: string | undefined = undefined;
    $: fontFamilyOverride = $ctx
        ? `${$ctx.google_font}, sans-serif`
        : undefined;
    $: (() => {
        if ($ctx !== undefined && !ignoreColor) {
            const c = new Color($ctx.primary_color);
            const isDark = isDarkMode();

            c.mix
            if (textLighter) {
                let color = c.clone();
                color.hsv.v = isDark ? 120 : 40;
                color.alpha = 0.7;
                textColorOverride = color.toString({ format: "hex" });
            } else {
                let color = c.clone();
                color.hsv.v = isDark ? 120 : 40;
                textColorOverride = color.toString({ format: "hex" });
            }
        } else {
            textColorOverride = undefined;
        }

        if (sizeGroup !== undefined) {
            const fsBase = getBaseREMFontSizeForBrand($ctx);
            let remSize: number;
            switch (sizeGroup) {
                case "h1+":
                    remSize = fsBase + 0.5;
                    break;
                case "h1":
                    remSize = fsBase + 0.25;
                    break;
                case "h2":
                    remSize = fsBase + 0.125;
                    break;
                case "p":
                    remSize = fsBase;
                    break;
            }

            fontSizeOverride = `${remSize}rem`;
        } else {
            fontSizeOverride = undefined;
        }
    })();
</script>

<svelte:head>
    {#if $ctx}
        <link
            href={`https://fonts.googleapis.com/css?family=${$ctx.google_font.replaceAll(" ", "+")}&display=block`}
            rel="stylesheet"
        />
    {/if}
</svelte:head>

<span
    style:font-family={fontFamilyOverride}
    style:font-size={fontSizeOverride}
    style:color={textColorOverride}
>
    <slot />
</span>
