const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./src/**/*.astro",
        "./src/**/*.svelte",
        "../../node_modules/flowbite-svelte/**/*.svelte",
        "../../node_modules/flowbite-svelte/**/*.ts",
        "../../node_modules/flowbite-svelte/**/*.js",
        "../frontend-common/src/**/*.svelte",
    ],
    darkMode: "media",
    theme: {
        extend: {
            colors: {
                primary: {
                    50: "#fcf3ff",
                    100: "#f7e6ff",
                    200: "#f0ccff",
                    300: "#e7a3ff",
                    400: "#db6dff",
                    500: "#cb44ff",
                    600: "#af16e3",
                    700: "#930ebd",
                    800: "#7a0e9a",
                    900: "#68117e",
                    950: "#430055",
                },
                samplePlayful: "#6560f7",
                sampleBrutal: "#fc3737",
            },
            fontFamily: {
                sans: ["Fira Code Variable", defaultTheme.fontFamily.sans],
                display: ["Archivo Variable", defaultTheme.fontFamily.sans],
                sampleElegant: "IBM Plex Serif",
                samplePlayful: "Nunito",
                sampleBrutal: "Kanit",
            },
        },
    },
};
