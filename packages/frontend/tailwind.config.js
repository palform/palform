const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{ts,svelte}",
    "../../node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],
  darkMode: "media",
  plugins: [require("flowbite/plugin")],
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
      },
      fontFamily: {
        display: ["Archivo Variable", ...defaultTheme.fontFamily.sans],
      },
    },
  },
};
