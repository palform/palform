/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  darkMode: ["class", '[data-theme="dark"]'],
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
    },
  },
  corePlugins: {
    preflight: false,
    // container: false,
  },
  plugins: [],
};
