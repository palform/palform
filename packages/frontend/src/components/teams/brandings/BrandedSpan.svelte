<script lang="ts">
    import {
        getBaseREMFontSizeForBrand,
        getBrandCtx,
    } from "../../../data/contexts/brand";
    import { isDarkMode } from "../../../data/util/darkMode";
    import { colorWithLightness } from "../../../data/util/color";

    const ctx = getBrandCtx();
    interface Props {
        textLighter?: boolean;
        ignoreColor?: boolean;
        sizeGroup?: "h1+" | "h1" | "h2" | "p" | undefined;
        children?: import("svelte").Snippet;
    }

    let {
        textLighter = false,
        ignoreColor = false,
        sizeGroup = undefined,
        children,
    }: Props = $props();

    let textColorOverride: string | undefined = $state(undefined);
    let fontSizeOverride: string | undefined = $state(undefined);
    let fontFamilyOverride = $derived(
        $ctx ? `${$ctx.google_font}, sans-serif` : undefined
    );

    $effect(() => {
        if ($ctx !== undefined && !ignoreColor) {
            const isDark = isDarkMode();

            if (textLighter) {
                textColorOverride = colorWithLightness(
                    $ctx.primary_color,
                    isDark ? 90 : 30,
                    0.7
                );
            } else {
                textColorOverride = colorWithLightness(
                    $ctx.primary_color,
                    isDark ? 70 : 15
                );
            }
        } else {
            textColorOverride = undefined;
        }

        if (sizeGroup !== undefined) {
            const fsBase = getBaseREMFontSizeForBrand($ctx);
            let remSize: number;
            switch (sizeGroup) {
                case "h1+":
                    remSize = fsBase * 1.5;
                    break;
                case "h1":
                    remSize = fsBase * 1.25;
                    break;
                case "h2":
                    remSize = fsBase * 1.175;
                    break;
                case "p":
                    remSize = fsBase;
                    break;
            }

            fontSizeOverride = `${remSize}rem`;
        } else {
            fontSizeOverride = undefined;
        }
    });
</script>

<svelte:head>
    {#if $ctx}
        <link
            href={`https://fonts.bunny.net/css?family=${$ctx.google_font.replaceAll(" ", "+")}&display=block`}
            rel="stylesheet"
        />
    {/if}
</svelte:head>

<span
    style:font-family={fontFamilyOverride}
    style:font-size={fontSizeOverride}
    style:color={textColorOverride}
>
    {@render children?.()}
</span>
