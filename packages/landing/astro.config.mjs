import { defineConfig } from "astro/config";
import icon from "astro-icon";
import svelte from "@astrojs/svelte";
import sitemap from "@astrojs/sitemap";
import tailwindcss from "@tailwindcss/vite";
import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
    integrations: [
        svelte(),
        icon(),
        sitemap({
            changefreq: "weekly",
        }),
    ],

    vite: {
        plugins: [tailwindcss()],
        // Required for Vite 8 + @tailwindcss/vite (createIdResolver expects resolve.tsconfigPaths).
        resolve: {
            tsconfigPaths: true,
        },
    },

    site: "https://palform.app",
    output: "static",
    adapter: node({
        mode: "standalone",
    }),
});
