import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";
import icon from "astro-icon";
import svelte from "@astrojs/svelte";
import sitemap from "@astrojs/sitemap";

// https://astro.build/config
export default defineConfig({
    integrations: [
        svelte(),
        tailwind(),
        icon(),
        sitemap({
            changefreq: "weekly",
        }),
    ],
    site: "https://palform.app",
});
