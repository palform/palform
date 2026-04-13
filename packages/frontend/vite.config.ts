import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasm from "vite-plugin-wasm";
import tailwindcss from "@tailwindcss/vite";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [wasm(), svelte(), tailwindcss()],
    worker: {
        plugins: () => [wasm()],
        format: "es",
    },
    build: {
        rollupOptions: {
            external: ["@flowbite-svelte-plugins/chart"]
        }
    }
});
