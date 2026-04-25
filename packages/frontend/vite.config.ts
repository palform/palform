import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasm from "vite-plugin-wasm";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath, URL } from "node:url";

// https://vitejs.dev/config/
export default defineConfig({
    resolve: {
        // Ensure packages like pouchdb-browser get the npm "events" polyfill
        // instead of Vite's browser-external stub for the Node builtin.
        alias: {
            events: fileURLToPath(
                new URL("./node_modules/events/events.js", import.meta.url)
            ),
            // Flowbite chart plugin currently imports `clsx` without declaring it,
            // so we pin resolution here to avoid unresolved bare imports in prod.
            clsx: fileURLToPath(
                new URL("./node_modules/clsx/dist/clsx.mjs", import.meta.url)
            ),
            "tailwind-merge": fileURLToPath(
                new URL(
                    "./node_modules/tailwind-merge/dist/bundle-mjs.mjs",
                    import.meta.url
                )
            ),
        },
    },
    plugins: [wasm(), svelte(), tailwindcss()],
    worker: {
        plugins: () => [wasm()],
        format: "es",
    },
});
