import defaultTheme from "tailwindcss/defaultTheme";

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class", "[data-theme='dark']"],
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      transitionProperty: {
        "colors-opacity": `${defaultTheme.transitionProperty.colors}, opacity`,
      },
      fontFamily: {
        sans: "'Geist', Arial, sans-serif",
      },
      colors: {
        accent: {
          light: "#1e40af",
          dark: "#7dd3fc",
        },
        border: {
          light: "#d4d4d4",
          dark: "#262626",
        },
        neutral: {
          250: "#dddddd",
          925: "#111111",
        },
      },
    },
  },
  plugins: [],
};
